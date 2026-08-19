// ============================================
// 第 15 课：模块 math（目录模块 src/math/mod.rs）
// ============================================

// 目录模块：用 mod.rs 或目录名.rs + 同名子目录两种方式
// 这里演示 mod.rs 方式；子模块 advanced 在 math/advanced.rs

// 声明子模块
pub mod advanced;

/// 加法
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 减法（私有辅助函数，仅 math 模块内可见）
fn sub(a: i32, b: i32) -> i32 {
    a - b
}

/// 公开减法：调用私有 sub
pub fn subtract(a: i32, b: i32) -> i32 {
    sub(a, b)
}
