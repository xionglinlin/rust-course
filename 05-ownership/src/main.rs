// ============================================
// 第 5 课：所有权（Ownership）—— Rust 的灵魂
// ============================================

// 三条规则：
// 1. Rust 中每个值都有一个"所有者"（owner）变量
// 2. 同一时刻只有一个所有者
// 3. 所有者离开作用域时，值被自动释放（drop）

fn main() {
    // ---------- 1. 栈 vs 堆 ----------
    // 栈：先进后出，快，存大小固定的值（整数、浮点、bool、char、定长数组）
    // 堆：手动分配，慢，存大小不固定的值（String、Vec）
    // Rust 没有 GC，靠"所有权"在编译期决定何时释放堆内存

    // ---------- 2. 移动（Move）----------
    // String 存在堆上。把它赋值给另一个变量，会发生"移动"：
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权被"移动"给了 s2

    // 下面这行取消注释会报错：value borrowed here after move
    // println!("{}", s1);
    // 因为 s1 已经不再拥有这个字符串了！
    println!("s2 = {}", s2); // ✅ s2 现在是唯一所有者

    // 为什么不是"拷贝"？—— 如果拷贝，两个变量指向同一块堆内存，
    // 作用域结束时会被释放两次（double free），造成内存安全问题。
    // Rust 的选择：移动 + 编译期禁止再使用旧变量，从根上杜绝 double free。

    // ---------- 3. 深拷贝（Clone）----------
    // 想真的复制一份内容？用 .clone()
    let a = String::from("你好");
    let b = a.clone(); // 堆上的内容被真正复制了一份
    println!("a = {}, b = {}", a, b); // 两个都能用 ✅

    // ---------- 4. Copy 类型：赋值是"复制"不是"移动"----------
    // 大小固定的类型（整数、浮点、bool、char、元组等）实现了 Copy trait
    // Copy 类型赋值后，旧变量依然可用
    let x = 5;
    let y = x; // 拷贝，x 依然有效
    println!("x = {}, y = {}", x, y); // ✅ 都能用

    // 注意：String 不实现 Copy（因为堆上数据不能简单复制），Vec 也不实现
    // 判断方法：如果类型实现了 Copy，赋值就是复制；否则就是移动

    // ---------- 5. 函数参数会"夺走"所有权 ----------
    let name = String::from("张三");
    take_ownership(name); // name 的所有权被移入函数
    // println!("{}", name); // ❌ 取消注释试试：use of moved value

    let num = 42;
    copy_me(num); // i32 是 Copy，传入的是副本，num 还能用
    println!("num 还能用: {}", num); // ✅

    // ---------- 6. 返回值可以"归还"所有权 ----------
    let s = gives_ownership(); // 函数返回的 String 所有权给了 s
    println!("s = {}", s);

    let t = takes_and_gives_back(s); // s 被移入，函数返回新 String 给 t
    // println!("{}", s); // ❌ s 已经没了
    println!("t = {}", t);

    // ---------- 7. 作用域结束自动释放 ----------
    // 每个 {} 是一个作用域，离开作用域时变量自动 drop
    {
        let temp = String::from("临时数据");
        println!("temp = {}", temp);
    } // 这里 temp 被自动释放
    // println!("{}", temp); // ❌ 超出作用域，编译错误

    // 小结：所有权规则让你无需手动 free/delete，
    // 编译器保证：内存不会泄漏（离开作用域必释放）、不会 double free（唯一所有者）、
    // 不会悬垂指针（旧变量编译期就被禁止使用）
}

fn take_ownership(s: String) {
    println!("函数里拿到了: {}", s);
} // 离开函数时 s 被释放，堆内存被回收

fn copy_me(n: i32) {
    println!("函数里拿到副本: {}", n);
}

fn gives_ownership() -> String {
    let s = String::from("由函数创建");
    s // 返回，所有权移给调用者
}

fn takes_and_gives_back(s: String) -> String {
    s // 原样返回，所有权归还调用者
}
