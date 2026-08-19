// ============================================
// 第 27 课：实战项目五 —— 迷你 Web 服务器
// ============================================
// 用标准库实现 HTTP 服务器：TCP 监听 + 手写线程池 + 解析 HTTP 请求。
// 这是 Rust 官方书 "Web Server" 章节的精华版。
//
// 综合运用：
//   - TcpListener/TcpStream（网络）     - 手写线程池（并发集大成）
//   - Box<dyn FnOnce>（trait 对象）     - Arc<Mutex> + mpsc（共享状态）
//   - Drop（优雅关闭）                  - 字符串处理（HTTP 协议）
//
// 运行：cargo run          （监听 127.0.0.1:7878）
// 测试：curl http://127.0.0.1:7878/
//       curl http://127.0.0.1:7878/sleep   （模拟慢请求）
//       curl http://127.0.0.1:7878/nope    （404）

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// ============================================
// 线程池：并发集大成（第 16/18/19/22 课的知识点全在这）
// ============================================
// Job = 一个可执行的闭包。
// Box<dyn FnOnce() + Send + 'static>：
//   - Box：堆分配（第 18 课）
//   - dyn FnOnce：trait 对象，运行时多态（第 18 课）
//   - Send：跨线程安全（第 19 课）
//   - 'static：闭包不能借用短期数据（第 14 课）
type Job = Box<dyn FnOnce() + Send + 'static>;

// 通道消息：NewJob 执行任务，Terminate 让 Worker 退出。
// 为什么不用"关闭通道"来通知退出？
// Drop 里 join 时 sender 字段还活着（字段在 Drop 之后才析构），
// Worker 会永远阻塞在 recv() → join 死锁。消息式关闭是正解。
enum Message {
    NewJob(Job),
    Terminate,
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0); // 至少一个线程

        let (sender, receiver) = mpsc::channel();
        // Arc<Mutex<Receiver>>：多个 Worker 共享同一个接收端
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    // 提交任务：闭包进通道，空闲 Worker 取走执行
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Message::NewJob(Box::new(f))).unwrap();
    }
}

// Drop：优雅关闭（给每个 Worker 发 Terminate，等它们退出）
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 1. 先给所有 Worker 发"退出"信号（此时 sender 还活着，消息能送达）
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        // 2. 再逐个 join：Worker 收到 Terminate 会 break，线程正常结束
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
                println!("Worker {} 已关闭", worker.id);
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // lock 拿接收端；阻塞等待消息
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} 执行任务", id);
                    job(); // 执行任务（闭包调用）
                }
                Message::Terminate => {
                    println!("Worker {} 收到退出信号", id);
                    break; // 退出循环，线程结束
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// ============================================
// HTTP 服务器
// ============================================
fn main() {
    // 监听 127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").expect("绑定端口失败");
    let pool = ThreadPool::new(4); // 4 个 Worker

    println!("服务器运行在 http://127.0.0.1:7878 （Ctrl+C 退出）");

    // 处理连接（demo 限制 10 个连接后退出，方便演示）
    for stream in listener.incoming().take(10) {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("连接失败: {}", e);
                continue;
            }
        };
        // 每个连接提交给线程池处理
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("已处理 10 个连接，服务器退出（线程池正在优雅关闭）");
}

// 处理单个 HTTP 连接
fn handle_connection(mut stream: TcpStream) {
    // 读取请求头（简化：最多 1024 字节）
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    if bytes_read == 0 {
        return; // 连接已关闭
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    // 请求行是第一行：GET /path HTTP/1.1
    let request_line = request.lines().next().unwrap_or("");

    println!("收到请求: {}", request_line);

    // 路由分发（用前缀匹配判断）
    let (status_line, contents) = if request_line.starts_with("GET / HTTP") {
        ("HTTP/1.1 200 OK", home_page())
    } else if request_line.starts_with("GET /sleep") {
        // 模拟慢请求：这个路由睡 2 秒
        thread::sleep(Duration::from_secs(2));
        ("HTTP/1.1 200 OK", sleep_page())
    } else {
        ("HTTP/1.1 404 NOT FOUND", not_found_page())
    };

    // 拼 HTTP 响应：状态行 + 头部 + 空行 + 正文
    let response = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// ---------- 页面内容 ----------
fn home_page() -> String {
    String::from(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>Rust 服务器</title></head>\
         <body><h1>🦀 你好，Rust 服务器！</h1>\
         <p>这个页面由手写线程池服务。</p>\
         <p><a href=\"/sleep\">访问 /sleep（模拟慢请求 2 秒）</a></p></body></html>",
    )
}

fn sleep_page() -> String {
    String::from(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"></head>\
         <body><h1>😴 睡了 2 秒</h1><p>这个请求故意慢了 2 秒。</p>\
         <p>再开一个终端访问 <code>/</code>，验证其他请求不被阻塞（线程池的功劳）</p>\
         <p><a href=\"/\">返回首页</a></p></body></html>",
    )
}

fn not_found_page() -> String {
    String::from(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"></head>\
         <body><h1>404</h1><p>页面不存在。</p><p><a href=\"/\">返回首页</a></p></body></html>",
    )
}
