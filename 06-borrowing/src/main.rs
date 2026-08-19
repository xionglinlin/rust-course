// ============================================
// 第 6 课：借用与引用（Borrowing & References）
// ============================================

// 上一课的痛点：函数借用 String 还要通过返回值归还所有权，太啰嗦。
// 解法：用引用 & 借用值，不转移所有权。
//
// 借用规则（由"借用检查器"borrow checker 强制）：
// 1. 同一时刻，要么有多个不可变借用 &，要么有一个可变借用 &mut（二选一）
// 2. 引用必须始终有效（不能悬垂）

fn main() {
    // ---------- 1. 不可变引用 & ----------
    // & 表示"借用"，只读，不拿走所有权
    let s = String::from("hello");
    let len = calculate_length(&s); // 传引用，s 的所有权不动
    println!("'{}' 的长度是 {}", s, len); // s 还能用 ✅

    // ---------- 2. 可变引用 &mut ----------
    let mut s2 = String::from("hello");
    change(&mut s2); // 传可变引用，函数里能改 s2
    println!("s2 被改成: {}", s2);

    // ---------- 3. 借用规则：不可变借用可以多个 ----------
    let x = String::from("共享");
    let r1 = &x;
    let r2 = &x; // 多个只读引用同时存在，OK
    println!("{} {}", r1, r2);

    // ---------- 4. 借用规则：可变借用同一时刻只能有一个 ----------
    let mut y = String::from("独占");
    let m1 = &mut y;
    // let m2 = &mut y; // ❌ 取消注释：cannot borrow `y` as mutable more than once
    println!("{}", m1);

    // 可变借用与不可变借用也不能同时存在
    let mut z = 10;
    let immut = &z; // 不可变借用开始
    println!("immut = {}", immut); // immut 最后一次使用
    // let mut_ref = &mut z; // ❌ 如果上面 println! 删掉，这行取消注释会报错：cannot borrow `z` as mutable because it is also borrowed as immutable
    let mut_ref = &mut z; // ✅ immut 的借用已结束，可变借用可以创建
    *mut_ref += 100;
    println!("z = {}", z);

    // 规则的本质：防止"数据竞争"（data race）——
    // 一个线程在写，另一个线程在读同一块内存，产生未定义行为。
    // Rust 在编译期就把这种可能性消灭了。

    // ---------- 5. 作用域：借用结束于最后一次使用 ----------
    let mut q = 5;
    let r = &q;          // 不可变借用开始
    println!("r = {}", r); // r 最后一次使用
    let m = &mut q;      // ✅ 可以了，r 的借用已经结束
    *m += 1;             // * 是解引用，通过引用修改值
    println!("q = {}", q);

    // ---------- 6. 悬垂引用（dangling reference）会被编译期拦截 ----------
    // 下面这个函数取消注释会报错：missing lifetime specifier / returns a reference to data owned by the current function
    // fn dangle() -> &String {
    //     let s = String::from("我会悬垂");
    //     &s // s 离开作用域被释放，返回的引用指向已释放内存！
    // }
    // 这正是 C/C++ 里最常见的 bug 之一，Rust 编译器直接拒绝编译。

    // ---------- 7. 解引用 * 与自动解引用 ----------
    let num = 100;
    let p = &num;
    println!("p 是指针 {:p}，解引用后 {}", p, *p); // {:p} 打印地址
    // 大多数情况下 Rust 会自动解引用，p 直接用就行
    println!("{}", p); // 自动解引用

    // ---------- 8. &str 字面量：也是引用 ----------
    // "hello" 的类型是 &'static str，本质是一个指向只读内存的引用
    let greeting: &str = "你好，Rust";
    println!("{}", greeting);

    // 小结：
    // - & 只读借用（可多个），&mut 写借用（唯一）
    // - 借用不转移所有权，用完后原变量照常可用
    // - 借用检查器在编译期杜绝：数据竞争、悬垂引用
}

// 参数 &String：只读借用。返回 usize，不需要归还所有权
fn calculate_length(s: &String) -> usize {
    s.len()
} // s 是借用，这里不释放，s 的所有者继续拥有它

// 参数 &mut String：可变借用，函数内可以修改
fn change(s: &mut String) {
    s.push_str(", world"); // 注意不需要解引用，方法调用会自动处理
}
