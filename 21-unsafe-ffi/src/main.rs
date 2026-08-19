// ============================================
// 第 21 课：unsafe Rust 与 FFI
// ============================================

// unsafe 不是"关闭安全检查"，而是"编译器放手，你负责保证安全"。
// unsafe 只给你 5 种超能力：
//   1. 解引用裸指针（*const T / *mut T）
//   2. 调用 unsafe 函数或方法（包括 extern FFI 函数）
//   3. 访问/修改可变静态变量（static mut）
//   4. 实现 unsafe trait
//   5. 访问 union 的字段
// 编译器依然检查其它所有内容！unsafe 是局部契约，不是全局豁免。

// ============ FFI 声明：调用 C 标准库 ============
// extern "C" 声明外部函数（ABI = C 调用约定）
// 本课链接 glibc（Rust std 已链接它），abs/printf 直接可用
// 注意：edition 2024 起 extern 块必须标注 unsafe（因为声明的外部函数不可信）
unsafe extern "C" {
    // 绝对值的 C 版本：int abs(int)
    fn abs(input: i32) -> i32;

    // C 的 printf：const char* 格式串 + 变参
    // 注意：C 字符串是以 \0 结尾的字节序列
    fn printf(format: *const u8, ...) -> i32;
}

fn main() {
    // ============ 1. 解引用裸指针 ============
    // 裸指针：*const T / *mut T，不遵守借用规则，编译器不检查
    let mut num = 5;

    // 从引用创建裸指针（安全操作）
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 解引用裸指针【必须】在 unsafe 块里
    unsafe {
        println!("r1 指向: {}", *r1);
        *r2 = 10; // 通过裸指针修改
    }
    println!("num 被裸指针改成: {}", num);

    // 危险示例：悬垂裸指针（编译器不拦！这就是 unsafe 的风险）
    let dangling: *const i32;
    {
        let temp = 42;
        dangling = &temp as *const i32;
    } // temp 已释放，dangling 悬垂
    // 只打印地址（不解引用，所以安全）；解引用则是未定义行为
    println!("悬垂指针的地址（不解引用）: {:p}", dangling);
    // unsafe { println!("{}", *dangling); } // ❌ 未定义行为！千万别取消注释

    // ============ 2. 调用 unsafe 函数 ============
    // 声明为 unsafe fn 的函数：只有 unsafe 块里能调
    unsafe {
        let n = dangerous_operation(10, 5);
        println!("unsafe 函数结果: {}", n);
    }

    // ============ 3. 可变静态变量 ============
    // 静态变量在程序全局共享，static mut 可能造成数据竞争 → 访问需 unsafe
    println!("当前调用次数: {}", unsafe { CALL_COUNT });
    unsafe {
        CALL_COUNT += 1;
    }
    println!("加一次后: {}", unsafe { CALL_COUNT });
    // 最佳实践：优先用原子类型 AtomicU32，不需要 unsafe
    use std::sync::atomic::{AtomicU32, Ordering};
    static SAFE_COUNT: AtomicU32 = AtomicU32::new(0);
    SAFE_COUNT.fetch_add(1, Ordering::SeqCst); // 安全！
    println!("原子计数（无需 unsafe）: {}", SAFE_COUNT.load(Ordering::SeqCst));

    // ============ 4. FFI：调用 C 库函数 ============
    println!("\n=== FFI 调用 C 标准库 ===");
    unsafe {
        // 调用 C 的 abs
        let c_abs = abs(-42);
        println!("C abs(-42) = {}", c_abs);

        // 调用 C 的 printf（b"..." 字节串字面量，\0 结尾；printf 格式串需 ASCII）
        let fmt = b"Output from C printf: %d\n\0";
        printf(fmt.as_ptr(), 2026);
    }

    // ============ 5. 安全封装 unsafe（最佳实践）============
    // unsafe 应该被"包"在安全接口里：unsafe 块尽可能小，外部调用者无需 unsafe
    let v = c_abs_safe(-99);
    println!("\n安全封装调用 C abs: {}", v);

    // ============ 6. 实现 unsafe trait ============
    // 大多数 trait 是安全的；有些（如 Send/Sync 的手工实现）是 unsafe trait
    // 因为"保证线程安全"是编译期无法验证的契约
    println!("--- 何时用 unsafe ---");
    println!("1. FFI：调用 C/C++ 代码（唯一正路）");
    println!("2. 极致性能：绕过借用检查的容器实现（如 Vec 内部）");
    println!("3. 与硬件交互：裸机/内核开发");
    println!("4. 自定义 Send/Sync、union 字段访问");
    println!("\n铁律：unsafe 越少越好；每个 unsafe 块都要写注释说明为什么安全");
}

// ============ unsafe 函数 ============
// 函数体里有 unsafe 操作时，函数本身标 unsafe：调用者也必须 unsafe
unsafe fn dangerous_operation(a: i32, b: i32) -> i32 {
    // 演示：故意绕过安全检查的场景（真实代码这里是 FFI 或性能关键路径）
    a.wrapping_mul(b) + a.wrapping_add(b)
}

// ============ static mut ============
static mut CALL_COUNT: u32 = 0;

// ============ 安全封装 FFI ============
// 外部调用者不用写 unsafe——不安全的部分被封装在这里
fn c_abs_safe(x: i32) -> i32 {
    unsafe { abs(x) } // unsafe 块最小化
}
