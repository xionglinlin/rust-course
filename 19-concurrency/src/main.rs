// ============================================
// 第 19 课：并发 —— 线程、消息传递、共享状态
// ============================================

// Rust 的并发哲学："无畏并发"（fearless concurrency）
// 数据竞争（data race）在编译期就被拒绝——第 6 课的借用规则在这里发威：
//   - Send trait：类型可以安全地跨线程转移所有权
//   - Sync trait：类型可以安全地被多线程共享引用
// 绝大多数类型自动满足，编译器在需要时强制检查

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // ============ 1. 创建线程：thread::spawn ============
    // 新线程运行闭包；主线程和新线程并发执行
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("子线程: {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..=3 {
        println!("主线程: {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    handle.join().unwrap(); // 等待子线程结束（阻塞直到完成）
    // 不 join 的话：主线程结束，程序退出，子线程可能没跑完

    // ============ 2. move 闭包：把数据移进线程 ============
    let data = vec![1, 2, 3];
    let t = thread::spawn(move || {
        // 没有 move 会编译错误：闭包可能比 data 活得更久
        println!("线程里拥有: {:?}", data);
    });
    // println!("{:?}", data); // ❌ data 已被 move 进线程
    t.join().unwrap();

    // ============ 3. 消息传递：mpsc 通道 ============
    // mpsc = multi-producer, single-consumer（多生产者，单消费者）
    let (tx, rx) = mpsc::channel();

    // 生产者线程：发送多条消息
    let tx1 = tx.clone(); // 通道可以克隆，多个生产者
    let producer1 = thread::spawn(move || {
        let msgs = vec![String::from("你好"), String::from("世界")];
        for m in msgs {
            tx1.send(m).unwrap(); // send 把消息【移动】进通道
            thread::sleep(Duration::from_millis(20));
        }
    });

    let producer2 = thread::spawn(move || {
        tx.send(String::from("来自第二个生产者")).unwrap();
    });

    // 消费者：主线程接收
    // rx 实现了迭代器，for 循环持续收到直到所有发送端关闭
    for received in rx {
        println!("收到: {}", received);
    }
    // 所有 tx 被 drop 后（生产者结束），for 循环自动结束

    producer1.join().unwrap();
    producer2.join().unwrap();

    // ============ 4. 共享状态：Arc<Mutex<T>> ============
    // Mutex：互斥锁，同一时刻只有一个线程能访问数据
    // Arc：原子引用计数（多线程版 Rc），多个线程共享所有权
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // 每个线程一份 Arc
        let h = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // 加锁，拿到可变引用
            *num += 1; // 临界区：修改共享数据
        }); // 锁在这里自动释放（num 离开作用域）
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("10 个线程各 +1，结果: {}", *counter.lock().unwrap()); // 10

    // 如果不用 Mutex：多个线程同时写会数据竞争（UB）。
    // Rust 的 Send/Sync 检查：Rc 不是 Send，跨线程编译直接报错！
    // let bad = Rc::new(0); thread::spawn(move || *bad); // ❌ Rc cannot be sent

    // ============ 5. 线程安全与 Send/Sync ============
    println!("--- Send/Sync 速查 ---");
    println!("i32, String, Vec: Send + Sync ✅");
    println!("Rc<T>: 非 Send（引用计数非原子）❌");
    println!("RefCell<T>: 非 Sync（运行时借用非线程安全）❌");
    println!("Arc<T>, Mutex<T>: Send + Sync ✅");
    println!("MutexGuard: 持有锁的守卫，自动解锁 ✅");

    // ============ 6. 并发实战：并行计算 ============
    // 用 4 个线程并行求和（分块）
    let numbers: Vec<i64> = (1..=100).collect();
    let num_threads = 4;
    let chunk_size = numbers.len() / num_threads;

    let mut handles = vec![];
    for i in 0..num_threads {
        let chunk: Vec<i64> = numbers[i * chunk_size..(i + 1) * chunk_size].to_vec();
        let h = thread::spawn(move || -> i64 {
            chunk.iter().sum() // 每个线程算一块
        });
        handles.push(h);
    }

    let total: i64 = handles
        .into_iter()
        .map(|h| h.join().unwrap()) // 收集每块结果
        .sum();
    println!("并行求和 1..=100 = {}", total); // 5050
    println!("(串行验证: {})", (1..=100).sum::<i64>());
}

// 注意：真实的并行计算有更优雅的工具（rayon 库），
// 但标准库 thread 是理解并发基础的最佳起点。
// 下一课异步（async/await）会有更轻量的并发模型。
