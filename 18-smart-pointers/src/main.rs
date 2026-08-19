// ============================================
// 第 18 课：智能指针 —— Box、Rc、RefCell、Arc
// ============================================

// 智能指针 = 拥有所有权 + 额外行为（引用计数、借用检查延迟等）的"指针"。
// 之前用过的 String、Vec 本质也是智能指针（拥有堆内存的所有权）。
// 本课四大天王：
//   Box<T>      → 堆上分配，所有权唯一（最基础）
//   Rc<T>       → 引用计数，单线程共享所有权
//   RefCell<T>  → 运行时借用检查，内部可变性
//   Arc<T>      → 引用计数，多线程共享（第 19 课详细用）

use std::cell::RefCell;
use std::rc::Rc;

// 递归类型必须用 Box（放在模块顶层，供函数使用）
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 链表递归求和
fn list_sum(list: &List) -> i32 {
    match list {
        List::Cons(value, rest) => value + list_sum(rest), // 递归
        List::Nil => 0, // 递归出口
    }
}

fn main() {
    // ============ 1. Box<T>：堆分配 ============
    // 为什么需要？① 递归类型必须用堆（否则无限大小）
    //              ② 大数据避免栈拷贝
    //              ③ trait 对象（dyn）
    let b = Box::new(5);
    println!("Box 里的值: {}", b); // 自动解引用
    println!("Box 解引用: {}", *b);

    // 递归类型示例：链表（不用 Box 会报 "recursive type has infinite size"）
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("链表: {:?}", list);
    println!("链表求和: {}", list_sum(&list)); // 递归遍历

    // 递归类型为什么必须用 Box？
    // List::Cons 包含 Box<List>：Box 是指针，大小固定；
    // 若直接写 Cons(i32, List)，编译器算不出 List 的大小（无限递归）→ 编译错误

    // ============ 2. Rc<T>：单线程共享所有权 ============
    // 一个值有多个所有者（只读共享）。用引用计数：最后一个离开时释放。
    // 注意：Rc 只读共享，不能修改（要修改用 RefCell 组合）
    let a = Rc::new(String::from("共享的字符串"));
    let b = Rc::clone(&a); // 引用计数 +1（不是深拷贝！）
    let c = Rc::clone(&a); // 引用计数再 +1
    println!("a: {}, b: {}, c: {}", a, b, c);
    println!("当前引用计数: {}", Rc::strong_count(&a)); // 3
    drop(c); // 手动释放一个引用
    println!("drop c 后计数: {}", Rc::strong_count(&a)); // 2

    // 多所有权实战：图/树中多个节点指向同一数据
    let shared = Rc::new(42);
    let node1 = (Rc::clone(&shared), 1);
    let node2 = (Rc::clone(&shared), 2);
    println!("node1: {:?}, node2: {:?}", node1, node2);

    // ============ 3. RefCell<T>：内部可变性 ============
    // 违反直觉但极有用：让"不可变引用"也能修改内部数据。
    // 借用检查从"编译期"挪到"运行时"——运行时违规会 panic。
    let cell = RefCell::new(String::from("hello"));
    // 借用规则仍有效：不可变借用和可变借用不能同时存在
    {
        let r1 = cell.borrow(); // 不可变借用
        println!("borrow: {}", r1);
    } // r1 作用域结束，借用释放
    cell.borrow_mut().push_str(", world"); // 可变借用
    println!("RefCell 修改后: {}", cell.borrow());

    // 运行时借用冲突 → panic（编译期检查不到的）
    // let m1 = cell.borrow_mut();
    // let m2 = cell.borrow_mut(); // ❌ 取消注释：already borrowed: BorrowMutError

    // ============ 4. Rc + RefCell：共享且可修改 ============
    // Rc 只读共享 + RefCell 内部可变 = 多个所有者都能改
    let shared_val = Rc::new(RefCell::new(10));
    let s1 = Rc::clone(&shared_val);
    let s2 = Rc::clone(&shared_val);

    *s1.borrow_mut() += 5; // 通过 s1 修改
    *s2.borrow_mut() += 10; // 通过 s2 修改
    println!("共享可改值: {}", shared_val.borrow()); // 25

    // ============ 5. Box<dyn Trait>：trait 对象（第 13 课的悬念）============
    // 运行时多态：不同类型装进同一个容器
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Rect { w: 2.0, h: 3.0 }),
    ];
    for s in &shapes {
        println!("面积: {:.2}", s.area());
    }
    // 对比：泛型是编译期确定类型（静态分派），dyn 是运行时（动态分派）
    // dyn 有少量运行时开销（虚表查找），换来灵活性

    // ============ 6. 智能指针对比总结 ============
    println!("--- 智能指针对比 ---");
    println!("Box: 唯一所有权，堆分配，静态分发");
    println!("Rc: 多所有权（单线程），只读共享");
    println!("RefCell: 运行时借用检查，内部可变");
    println!("Arc: 多所有权（多线程），第 19 课细讲");
}

// ---------- trait 对象示例 ----------
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rect {
    w: f64,
    h: f64,
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}
