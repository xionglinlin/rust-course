// ============================================
// 第 2 课：变量与数据类型
// ============================================

fn main() {
    // ---------- 1. 变量默认不可变 ----------
    // Rust 中变量默认是"不可变"的（immutable）
    let x = 5;
    println!("x = {}", x);

    // 下面的代码会编译报错！x 不可变
    // x = 6;  // 取消注释试试看：error[E0384]: cannot assign twice

    // 用 `mut` 关键字声明可变变量
    let mut y = 5;
    println!("y 一开始是 {}", y);
    y = 6; // OK，y 是可变的
    println!("y 现在是 {}", y);

    // ---------- 2. 常量 ----------
    // `const` 声明常量：必须标注类型，且值必须是编译期常量
    // 常量命名用全大写 + 下划线
    const MAX_POINTS: u32 = 100_000; // 数字可以用下划线分隔，方便阅读
    println!("MAX_POINTS = {}", MAX_POINTS);

    // ---------- 3. 遮蔽（Shadowing）----------
    // 可以用同名的新变量"遮蔽"旧变量，类型还可以不同
    let s = "   ";
    let s = s.len(); // 遮蔽：s 从 &str 变成 usize
    println!("s 被遮蔽后 = {}", s);

    // ---------- 4. 整数类型 ----------
    // 有符号 i8/i16/i32/i64/i128/isize，无符号 u8/u16/u32/u64/u128/usize
    let a: i32 = -42;      // 有符号 32 位
    let b: u8 = 255;       // 无符号 8 位，范围 0~255
    let c = 1_000_000u64;  // 后缀标注类型
    println!("a={}, b={}, c={}", a, b, c);

    // 整数溢出（debug 模式下会 panic，release 下回绕）
    // let overflow: u8 = 256; // 编译错误：literal out of range

    // ---------- 5. 浮点类型 ----------
    let pi: f64 = 3.14159; // 默认是 f64
    let e: f32 = 2.71828;  // 也可以显式用 f32
    println!("pi={}, e={}", pi, e);

    // ---------- 6. 布尔类型 ----------
    let is_rust_fun = true;
    let is_hard = false;
    println!("Rust 有趣吗? {}", is_rust_fun);
    println!("Rust 难吗? {}", is_hard);

    // ---------- 7. 字符类型 ----------
    // char 是 4 字节的 Unicode 标量值，可以表示中文、emoji
    let c1 = 'R';
    let c2 = '中';
    let c3 = '😀';
    println!("char: {} {} {}", c1, c2, c3);

    // ---------- 8. 元组（Tuple）----------
    // 不同类型的值打包在一起，长度固定
    let tup: (i32, f64, char) = (500, 6.4, 'X');
    // 解构
    let (t0, t1, t2) = tup;
    println!("解构: {} {} {}", t0, t1, t2);
    // 用 . 索引访问
    println!("索引: {} {}", tup.0, tup.1);

    // ---------- 9. 数组（Array）----------
    // 同类型、长度固定，存在栈上
    let arr = [1, 2, 3, 4, 5];
    println!("arr 第一个元素 = {}", arr[0]);
    println!("arr 的长度 = {}", arr.len());
    // 越界访问会 panic（运行时崩溃），而不是 UB
    // println!("{}", arr[10]); // 取消注释：index out of bounds

    // 声明固定长度的数组
    let zeros: [i32; 5] = [0; 5]; // 5 个 0
    println!("zeros = {:?}", zeros);

    // ---------- 10. 类型推断 ----------
    // Rust 会根据使用方式推断类型
    let guess = "42".parse::<i32>().unwrap();
    println!("guess = {} (类型是 i32)", guess);
}
