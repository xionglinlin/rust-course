// ============================================
// 第 30 课：毕业项目 —— 能力自检与生态巡礼
// ============================================
// 30 课学完了，用一个"能力自检"程序回顾所有核心概念。
// 每个 check 函数考察一类知识，全部通过 → 恭喜毕业！
//
// 运行：cargo run
// 期待输出：10 项能力检查全部 ✅

// ---------- 自检框架 ----------
struct Check {
    name: &'static str,
    passed: bool,
}

fn run_all() -> Vec<Check> {
    vec![
        Check { name: "基础语法：变量/类型/控制流", passed: check_basics() },
        Check { name: "所有权与借用", passed: check_ownership() },
        Check { name: "结构体/枚举/模式匹配", passed: check_data_modeling() },
        Check { name: "集合与迭代器", passed: check_collections() },
        Check { name: "错误处理与 Result", passed: check_error_handling() },
        Check { name: "泛型与 trait", passed: check_generics_traits() },
        Check { name: "闭包与高阶函数", passed: check_closures() },
        Check { name: "智能指针", passed: check_smart_pointers() },
        Check { name: "并发", passed: check_concurrency() },
        Check { name: "宏与元编程", passed: check_macros() },
    ]
}

// ---------- 各能力检查 ----------
fn check_basics() -> bool {
    // 变量、类型、控制流、函数（第 1-4 课）
    let mut total = 0;
    for i in 1..=100 {
        total += i; // 1+2+...+100 = 5050
    }
    let grade = if total > 5000 { "A" } else { "F" };
    let tuple: (i32, f64, char) = (total, 3.14, '🦀');
    total == 5050 && grade == "A" && tuple.0 == 5050
}

fn check_ownership() -> bool {
    // 移动、克隆、借用（第 5-6 课）
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 克隆：两个都有效
    let len = {
        let borrowed = &s2; // 借用
        borrowed.len()
    };
    s1 == s2 && len == 5 && s1.len() == s2.len()
}

fn check_data_modeling() -> bool {
    // 枚举 + match（第 8-9 课）
    enum Shape {
        Circle(f64),
        Rect(f64, f64),
    }
    fn area(s: &Shape) -> f64 {
        match s {
            Shape::Circle(r) => 3.14159 * r * r,
            Shape::Rect(w, h) => w * h,
        }
    }
    let c = Shape::Circle(1.0);
    let r = Shape::Rect(2.0, 3.0);
    (area(&c) - 3.14159).abs() < 1e-9 && area(&r) == 6.0
}

fn check_collections() -> bool {
    // Vec + HashMap + 迭代器（第 10、17 课）
    use std::collections::HashMap;
    let nums: Vec<i32> = (1..=10).filter(|x| x % 2 == 0).map(|x| x * x).collect();
    let mut map = HashMap::new();
    for n in &nums {
        *map.entry(n).or_insert(0) += 1;
    }
    nums.len() == 5 && map.values().all(|&v| v == 1) && nums.iter().sum::<i32>() == 220
}

fn check_error_handling() -> bool {
    // Result + ?（第 11 课）
    fn safe_div(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("除零"))
        } else {
            Ok(a / b)
        }
    }
    fn chain(a: i32, b: i32) -> Result<i32, String> {
        let v = safe_div(a, b)?; // ? 传播错误
        Ok(v * 2)
    }
    chain(10, 2) == Ok(10) && chain(1, 0).is_err()
}

fn check_generics_traits() -> bool {
    // 泛型 + trait（第 12-13 课）
    trait Describe {
        fn describe(&self) -> String;
    }
    struct Point {
        x: i32,
        y: i32,
    }
    impl Describe for Point {
        fn describe(&self) -> String {
            format!("({}, {})", self.x, self.y)
        }
    }
    fn print_describe<T: Describe>(item: &T) -> String {
        item.describe()
    }
    let p = Point { x: 1, y: 2 };
    print_describe(&p) == "(1, 2)"
}

fn check_closures() -> bool {
    // 闭包 + 捕获（第 16 课）
    let base = 100;
    let adder = |x: i32| x + base; // 捕获环境
    let doubled: Vec<i32> = vec![1, 2, 3].into_iter().map(|x| x * 2).collect();
    adder(5) == 105 && doubled == vec![2, 4, 6]
}

fn check_smart_pointers() -> bool {
    // 智能指针（第 18 课）
    use std::cell::RefCell;
    use std::rc::Rc;

    let shared = Rc::new(RefCell::new(10));
    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);
    *a.borrow_mut() += 5;
    *b.borrow_mut() += 5;
    *shared.borrow() == 20 && Rc::strong_count(&shared) == 3
}

fn check_concurrency() -> bool {
    // 并发（第 19 课）
    use std::sync::{Arc, Mutex};
    use std::thread;

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..4 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut n = c.lock().unwrap();
            *n += 1;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    *counter.lock().unwrap() == 4
}

// 宏：声明式宏（第 22 课）
macro_rules! assert_pass {
    ($cond:expr) => {
        $cond
    };
}

fn check_macros() -> bool {
    let three = 3;
    assert_pass!(three == 3) && assert_pass!(vec![1, 2, 3].len() == 3)
}

// ---------- main ----------
fn main() {
    println!("════════════════════════════════════════════════");
    println!("  🎓 Rust 30 课毕业自检");
    println!("════════════════════════════════════════════════\n");

    let checks = run_all();
    let passed_count = checks.iter().filter(|c| c.passed).count();

    for check in &checks {
        let mark = if check.passed { "✅" } else { "❌" };
        println!("  {} {}", mark, check.name);
    }

    println!("\n════════════════════════════════════════════════");
    if passed_count == checks.len() {
        println!("  🎉 {}/{} 通过 —— 恭喜，基础到进阶的核心能力已就位！", passed_count, checks.len());
    } else {
        println!("  {}/{} 通过 —— 有 {} 项没过，复习对应课程", passed_count, checks.len(), checks.len() - passed_count);
    }
    println!("════════════════════════════════════════════════");

    // ---------- 生态巡礼 ----------
    println!("\n🌍 Rust 生态地图（下一步学什么）");
    println!("────────────────────────────────────────");
    let ecosystem = [
        ("Web/异步", "tokio, axum, actix-web", "第 20 课 async 的实战延伸"),
        ("序列化", "serde + serde_json", "第 26 课 JSON 解析器的工业级版"),
        ("CLI", "clap, structopt", "第 23-24 课命令行工具的专业化"),
        ("并行计算", "rayon", "第 25 课 map-reduce 的一行版"),
        ("测试", "criterion, proptest", "单元测试的进阶：基准与属性测试"),
        ("数据库", "sqlx, diesel, rusqlite", "类型安全的 SQL"),
        ("GUI", "egui, iced, slint", "桌面应用"),
        ("游戏", "bevy", "ECS 架构游戏引擎"),
        ("嵌入式", "embedded-hal, esp-rs", "裸机/Rust 上芯片"),
        ("操作系统", "Linux 内核(部分), Redox", "系统级终极目标"),
    ];
    for (area, crates, link) in ecosystem {
        println!("  {:<6} {:<28} ← {}", area, crates, link);
    }

    println!("\n📚 推荐下一步资源");
    println!("  · The Rust Book（官方书，本课程主干来源）");
    println!("  · Rust By Example（按例子学）");
    println!("  · Rustlings（交互式练习）");
    println!("  · 各 crate 的官方文档（docs.rs）");

    println!("\n所有 demo 都在 rust-course/ 目录下，随时可以重跑复习。");
    println!("祝你在 Rust 的世界里写出安全、高性能的代码！🦀");
}
