// ============================================
// 第 15 课：子模块 math::advanced（src/math/advanced.rs）
// ============================================

/// 幂运算：base 的 exp 次方
pub fn power(base: f64, exp: f64) -> f64 {
    base.powf(exp)
}

/// 平方根
pub fn sqrt(x: f64) -> f64 {
    x.sqrt()
}
