// ============================================
// 第 15 课：模块与包管理 —— 库 crate（lib.rs）
// ============================================

// 一个项目可以有两个"入口"：
// - main.rs → 二进制 crate（可执行文件）
// - lib.rs  → 库 crate（供别人/自己 use）
// 本项目两者都有：main.rs 作为可执行程序，使用 lib.rs 里的模块

// 声明模块：告诉编译器"这些模块存在"
// 对应文件：src/greeting.rs、src/math/mod.rs
pub mod greeting;
pub mod math;

// 库顶层的公开函数
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

// pub(crate) 项在同一 crate 内的其它模块可以访问（跨模块可见）：
// greeting::internal_hint() 是 pub(crate)，lib.rs 属于同一个 crate → 可用
pub fn greeting_hint() -> String {
    greeting::internal_hint().to_string()
}

// 模块树（逻辑结构）：
// modules (lib.rs)
// ├── greeting (greeting.rs)
// └── math (math/mod.rs)
//     └── advanced (math/advanced.rs)
