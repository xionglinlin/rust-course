// ============================================
// 第 9 课：枚举 enum 与模式匹配 match
// ============================================

// 枚举：一个类型可以"是"多个变体（variant）之一
// 相比 C 的 enum（只是整数），Rust 的枚举可以携带数据！

// ---------- 1. 最简单的枚举 ----------
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

// ---------- 2. 携带数据的枚举 ----------
// 每个变体可以带不同类型、不同数量的数据
#[derive(Debug)]
enum Message {
    Quit,                       // 不带数据
    Move { x: i32, y: i32 },    // 带具名字段（类似结构体）
    Write(String),              // 带一个 String
    ChangeColor(u8, u8, u8),    // 带三个值（类似元组结构体）
}

// ---------- 3. Option<T>：Rust 里没有 null！----------
// 标准库最重要的枚举：
// enum Option<T> { None, Some(T) }
// 表达"可能有值，可能没有"——杜绝空指针异常
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None // 除零：没有结果
    } else {
        Some(a / b) // 有结果
    }
}

fn main() {
    // ---------- 使用枚举 ----------
    let dir = Direction::North;
    println!("方向: {:?}", dir);
    // 构造其余变体，避免 dead_code 警告
    let _all = [Direction::East, Direction::West];
    describe_direction(Direction::East);

    let msg = Message::Write(String::from("你好"));
    let msg2 = Message::Move { x: 10, y: -5 };
    let msg3 = Message::ChangeColor(255, 0, 128);
    println!("{:?} {:?} {:?}", msg, msg2, msg3);
    process_message(Message::Quit); // 构造 Quit 变体

    // ---------- match：模式匹配 ----------
    // match 是 Rust 的 switch 增强版：
    // 1. 必须穷尽所有可能（少一个分支就编译失败！）
    // 2. 每个分支是表达式，可以返回值
    // 3. 从上往下匹配，第一个命中的生效
    describe_direction(Direction::North);
    describe_direction(Direction::South);

    // match 也是表达式，可以赋值
    let score = 88;
    let grade = match score {
        90..=100 => "A", // 范围模式
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",        // 通配符：匹配所有剩余情况
    };
    println!("{} 分是 {} 等级", score, grade);

    // ---------- 处理 Option ----------
    let r1 = divide(10.0, 2.0);
    let r2 = divide(10.0, 0.0);

    match r1 {
        Some(v) => println!("10/2 = {}", v),
        None => println!("除零了！"),
    }
    match r2 {
        Some(v) => println!("10/0 = {}", v),
        None => println!("除零了！"),
    }

    // ---------- if let：只想匹配一种情况时的简写 ----------
    // 完整 match 要求穷尽，如果只关心一个变体，用 if let
    let config = Some(3u8);
    if let Some(max) = config {
        println!("最大连接数设置为 {}", max);
    } else {
        println!("未设置连接数");
    }

    // ---------- 解构：从枚举/结构体里取出数据 ----------
    process_message(msg);
    process_message(msg2);
    process_message(msg3);

    // ---------- 解构结构体 ----------
    let point = Point { x: 3, y: 7 };
    match point {
        Point { x, y } => println!("点的坐标: ({}, {})", x, y),
    }
    // 只关心部分字段
    match point {
        Point { y, .. } => println!("只看 y: {}", y),
    }

    // ---------- 绑定值 @ ----------
    let n = 5;
    match n {
        0 => println!("零"),
        small @ 1..=9 => println!("个位数: {}", small), // @ 把匹配值绑定到变量
        big @ _ => println!("大数: {}", big),
    }

    // ---------- 常见模式：unwrap / unwrap_or ----------
    // 简化 Option 处理（错误处理课会深入）
    let r = divide(8.0, 4.0);
    println!("直接取: {}", r.unwrap()); // 有值就取，None 则 panic
    let safe = divide(1.0, 0.0).unwrap_or(-1.0); // None 时给默认值
    println!("安全取: {}", safe);
}

// 用 match 处理方向，演示穷尽性
fn describe_direction(d: Direction) {
    match d {
        Direction::North => println!("向北走"),
        Direction::South => println!("向南走"),
        Direction::East => println!("向东走"),
        Direction::West => println!("向西走"),
        // 注释掉任何一个分支试试：编译器报 "non-exhaustive patterns"
    }
}

// 用 match 解构 Message 的各种数据
fn process_message(m: Message) {
    match m {
        Message::Quit => println!("退出消息"),
        Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
        Message::Write(text) => println!("写入: {}", text),
        Message::ChangeColor(r, g, b) => println!("颜色: rgb({}, {}, {})", r, g, b),
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
