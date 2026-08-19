// ============================================
// 第 3 课：函数 —— 参数、返回值、语句与表达式
// ============================================

fn main() {
    // 调用函数
    greet("小明");

    let s = add(3, 5);
    println!("3 + 5 = {}", s);

    // 函数可以嵌套调用
    let result = add(add(1, 2), add(3, 4));
    println!("(1+2) + (3+4) = {}", result);

    // ---------- 语句 vs 表达式 ----------
    // 语句（statement）：执行操作但不返回值，如 let x = 5;
    // 表达式（expression）：会计算出值
    // 在 Rust 里，"一切皆表达式"，{} 块也是表达式，其值是最后一个表达式

    let y = {
        let a = 10;
        let b = 20;
        a + b // 注意：没有分号！这是块表达式的值
    };
    println!("块表达式的值 y = {}", y);

    // ---------- 函数作为"表达式"的返回值 ----------
    let doubled = double(7);
    println!("7 翻倍 = {}", doubled);

    // ---------- 多个返回值：用元组 ----------
    let (sum, product) = sum_and_product(3, 4);
    println!("和 = {}, 积 = {}", sum, product);

    // ---------- 提前返回 ----------
    let r = early_return(true);
    println!("early_return(true) = {}", r);
    let r2 = early_return(false);
    println!("early_return(false) = {}", r2);
}

// 无返回值（返回单元类型 ()）
// 参数必须标注类型
fn greet(name: &str) {
    println!("你好，{}！", name);
}

// 有返回值：用 -> 指定返回类型
// 最后一个表达式（无分号）就是返回值
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 也可以用 return 关键字显式返回（一般用于提前返回）
fn double(x: i32) -> i32 {
    return x * 2; // 显式 return
}

// 返回元组，实现"多返回值"
fn sum_and_product(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}

// 提前返回
fn early_return(flag: bool) -> i32 {
    if flag {
        return 1; // 提前返回
    }
    0 // 正常走到最后
}
