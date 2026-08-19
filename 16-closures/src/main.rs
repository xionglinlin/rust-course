// ============================================
// 第 16 课：闭包 Closures —— 能捕获环境变量的匿名函数
// ============================================

// 闭包 = 匿名函数 + 捕获环境（外层作用域的变量）
// 语法：|参数| 表达式 或 |参数| { 多行代码 }

fn main() {
    // ---------- 1. 闭包的基本语法 ----------
    // 函数 vs 闭包对比：
    fn add_fn(a: i32, b: i32) -> i32 { a + b } // 函数：不能捕获环境
    let add_cl = |a: i32, b: i32| a + b;       // 闭包：可以捕获环境
    println!("函数: {}", add_fn(1, 2));
    println!("闭包: {}", add_cl(1, 2));

    // 闭包的类型可以推断，参数不写类型也行
    let mul = |a, b| a * b; // 类型由第一次调用推断
    println!("乘法闭包: {}", mul(3, 4));
    // 一旦推断出类型，就不能换类型：
    // println!("{}", mul(3.5, 2.0)); // ❌ 已经是 i32 的闭包了

    // ---------- 2. 捕获环境：闭包与函数的本质区别 ----------
    let base = 10;
    let add_base = |x| x + base; // 捕获了 base！
    println!("10 + 5 = {}", add_base(5));
    // 函数做不到：fn f(x: i32) -> i32 { x + base } // ❌ base 不在作用域

    // ---------- 3. 三种捕获方式（对应三种 trait）----------
    // ① Fn：不可变借用捕获（只读）
    let msg = String::from("你好");
    let print_msg = || println!("{}", msg); // Fn：借用 msg
    print_msg();
    print_msg(); // 可多次调用
    println!("msg 还能用: {}", msg); // ✅ 只是借用

    // ② FnMut：可变借用捕获（可改）
    let mut count = 0;
    let mut increment = || {
        count += 1; // 捕获 &mut count
    };
    increment();
    increment();
    println!("count = {}", count); // ✅ 借用结束，count 可用

    // ③ FnOnce：获取所有权捕获（拿走）
    let name = String::from("小明");
    let consume = || {
        drop(name); // 拿走 name 的所有权（可以 drop 它）
    };
    consume();
    // consume(); // ❌ name 已被消耗，闭包不能再调用
    // println!("{}", name); // ❌ name 被闭包拿走了

    // 选择原则：能用 Fn 就不用 FnMut，能用 FnMut 就不用 FnOnce
    // 越宽松的捕获，闭包能被使用的地方越多

    // ---------- 4. move 关键字：强制拿走所有权 ----------
    // 典型场景：新线程需要拥有数据（第 19 课并发会用到）
    let data = vec![1, 2, 3];
    let moved = move || {
        // 如果没有 move，闭包只借用 data；但线程可能活得比 main 久
        println!("move 闭包拥有: {:?}", data);
    };
    // println!("{:?}", data); // ❌ data 已被 move 进闭包
    moved();

    // ---------- 5. 闭包作为参数（泛型约束）----------
    // F: Fn(i32) -> i32 意思是"F 是一个能接收 i32 返回 i32 的闭包/函数"
    fn apply_twice<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(f(x))
    }
    let double = |x| x * 2;
    println!("double(double(5)) = {}", apply_twice(double, 5));
    // 函数也能传（函数实现了所有 Fn trait）
    println!("abs 两次 = {}", apply_twice(i32::abs, -3));

    // ---------- 6. 闭包作为返回值 ----------
    // 返回闭包：impl Fn 语法（不能直接写闭包类型）
    fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
        move |y| x + y // move：把 x 移进闭包，闭包独立存活
    }
    let add5 = make_adder(5);
    let add100 = make_adder(100);
    println!("add5(10) = {}, add100(10) = {}", add5(10), add100(10));
    // 每次调用 make_adder 都生成一个新的闭包，捕获各自的 x

    // ---------- 7. 实战：用闭包做配置 ----------
    let price = 100;
    // 一个"折扣生成器"：折扣率不同，闭包不同
    let discount_10 = make_discount(0.1);
    let discount_50 = make_discount(0.5);
    println!("原价 {}：打9折 = {}", price, discount_10(price));
    println!("原价 {}：打5折 = {}", price, discount_50(price));
}

// 闭包实战：折扣生成器（返回闭包）
fn make_discount(rate: f64) -> impl Fn(i32) -> i32 {
    move |price| (price as f64 * (1.0 - rate)) as i32
}
