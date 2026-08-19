// ============================================
// 第 20 课：异步编程 async/await —— 纯标准库手写 mini executor
// ============================================

// 为什么需要异步？
// 线程是操作系统级的：每个线程有独立栈（~8MB），创建/切换开销大。
// 异步 = 用户态协作式调度：一个线程上跑成千上万个"任务"，遇到 I/O
// 就主动让出（yield），不阻塞线程——高并发 I/O 场景的标配。
//
// 核心概念：
//   Future      → 一个"待完成的计算"，惰性的，poll 它才会推进
//   poll()      → 推进一步；返回 Ready(值) 或 Pending(还没好)
//   async fn    → 语法糖：把函数体编译成一个 Future 状态机
//   .await      → 暂停当前 Future，等待另一个 Future 完成
//   Waker       → 完成时唤醒 executor 重新 poll（本课用空实现）
//   executor    → 驱动 poll 的循环（tokio/async-std 是生产级 executor）

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

// ---------- 迷你 executor：block_on ----------
// 生产级 executor（tokio）用事件循环 + 线程池；这里用最简单的忙轮询演示原理
fn block_on<F: Future>(fut: F) -> F::Output {
    // Box::pin 把 Future 固定到堆上（Future 必须是 !Unpin 安全地 poll）
    let mut fut = Box::pin(fut);

    // 空 Waker：本课所有 Future 都能立即完成，不需要真正的唤醒机制
    let waker = Waker::from(Arc::new(NoopWaker));
    let mut cx = Context::from_waker(&waker);

    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(value) => return value, // 完成了！
            Poll::Pending => {
                // 还没完成：让出 CPU 再试。
                // 生产级 executor 会在这里挂起，等 Waker 唤醒。
                std::thread::yield_now();
            }
        }
    }
}

struct NoopWaker;

// Wake trait：实现唤醒行为（std 提供的安全接口）
impl Wake for NoopWaker {
    fn wake(self: Arc<Self>) {
        // 空操作：忙轮询模式下不需要真正唤醒
    }
}

// ---------- 自定义 Future：返回 Pending 若干次的"异步等待" ----------
// 模拟一个需要时间才能完成的操作（真实世界是网络/文件 I/O）
struct CountDown(u32);

impl Future for CountDown {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.0 == 0 {
            println!("  CountDown 完成！");
            Poll::Ready(())
        } else {
            self.0 -= 1;
            println!("  CountDown 还没好 (剩 {})，返回 Pending", self.0);
            Poll::Pending
        }
    }
}

// ---------- async fn：语法糖 ----------
// async fn 调用时【不执行】！它返回一个 Future，poll 才会跑函数体
async fn fetch_data(id: u32) -> String {
    println!("开始获取数据 {}", id);
    // 模拟异步等待（真实场景是 I/O）
    CountDown(3).await;
    format!("数据 {}", id)
}

async fn process_data() -> String {
    // .await 暂停当前 Future，等待 fetch_data 完成
    let a = fetch_data(1).await;
    let b = fetch_data(2).await;
    // 顺序 await：并发版本见下方
    format!("{} + {}", a, b)
}

// ---------- 并发 await：join 式执行 ----------
// 用 futures 库的 join! 宏最优雅；这里手写两个 Future 轮流 poll 模拟并发
struct Join2<A, B>(A, B);

impl<A: Future, B: Future> Future for Join2<A, B> {
    type Output = (A::Output, B::Output);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 安全：我们只在 &mut 上操作，且不移动内部字段
        let this = unsafe { self.get_unchecked_mut() };
        let a = unsafe { Pin::new_unchecked(&mut this.0) };
        let b = unsafe { Pin::new_unchecked(&mut this.1) };

        // 轮流推进：任何一个 Pending 就整体 Pending
        let a_ready = match a.poll(cx) {
            Poll::Ready(v) => Some(v),
            Poll::Pending => None,
        };
        let b_ready = match b.poll(cx) {
            Poll::Ready(v) => Some(v),
            Poll::Pending => None,
        };

        match (a_ready, b_ready) {
            (Some(va), Some(vb)) => Poll::Ready((va, vb)),
            _ => Poll::Pending,
        }
    }
}

fn main() {
    // ---------- 1. async fn 是惰性的 ----------
    println!("=== 1. async fn 惰性 ===");
    let fut = fetch_data(0); // 只是创建 Future，函数体【还没执行】
    println!("创建 Future 后，函数体没执行（无输出）");
    block_on(fut); // poll 之后才开始执行

    // ---------- 2. 顺序 await ----------
    println!("\n=== 2. 顺序 await ===");
    let result = block_on(process_data());
    println!("顺序结果: {}", result);

    // ---------- 3. 并发 await（手写 Join2）----------
    println!("\n=== 3. 并发 await（Join2 轮流 poll）===");
    let concurrent = block_on(Join2(fetch_data(10), fetch_data(20)));
    println!("并发结果: {:?}", concurrent);

    // ---------- 4. 概念对照 ----------
    println!("\n=== 4. 概念对照 ===");
    println!("Future = 惰性状态机（async fn 编译产物）");
    println!("poll = 推进一步，Ready/Pending 二选一");
    println!("await = 暂停自己，等待别人");
    println!("Waker = 完成时叫醒 executor");
    println!("executor = 调度 poll 的循环（tokio 是生产级实现）");
    println!("\n生产实践：写 tokio（网络、数据库、文件）时，你写的还是");
    println!("同样的 async fn + await，只是 executor 换成 tokio 的。");
}
