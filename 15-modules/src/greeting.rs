// ============================================
// 第 15 课：模块 greeting（文件模块 src/greeting.rs）
// ============================================

// 可见性：
// - pub        → 公开，外部可用
// - pub(crate) → 仅本 crate 内可见（二进制 + 库内部）
// - 默认(无 pub) → 仅本模块及其子模块可见

/// 生成问候语（/// 是文档注释，会生成文档）
pub fn hello(name: &str) -> String {
    format!("你好，{}！", name)
}

/// 再见（pub：外部 crate 也可用）
pub fn bye(name: &str) -> String {
    format!("再见，{}！", name)
}

/// 内部提示：pub(crate) 只在【本 crate 内】可见。
/// 注意：lib.rs 的库和 main.rs 的二进制是【两个不同 crate】，
/// 所以二进制的 main.rs 用不了 pub(crate) 的项（会报 E0603）。
/// 但它能被同 crate 的其它模块用——见 lib.rs 的 greeting_hint()
pub(crate) fn internal_hint() -> &'static str {
    "内部信息：本提示只在库 crate 内可见"
}

/// 私有函数：只有 greeting 模块内能用
fn secret() -> String {
    String::from("这是 greeting 模块的秘密")
}

// 模块内可以自由用私有函数
pub fn secret_message() -> String {
    secret()
}
