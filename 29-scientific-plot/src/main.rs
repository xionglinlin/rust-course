// ============================================
// 第 29 课：实战项目七 —— 科学计算与 SVG 绘图
// ============================================
// 数值积分（梯形法 + 辛普森法）+ 函数曲线 SVG 绘图。
// 展示 Rust 在数值计算方向的能力：闭包当数学函数、泛型算法、
// 高精度浮点、无 GC 的性能优势。
//
// 用法：
//   cargo run                # 积分演示 + 生成 SVG 图
//   cargo test               # 数值方法正确性测试
//
// 输出：sin_curve.svg 等 SVG 文件（可用浏览器打开）

use std::f64::consts::PI;
use std::fs;

// ============================================
// 一、数值积分
// ============================================

// 闭包 F: Fn(f64) -> f64 就是"数学函数"的完美建模
// 梯形法：把区间 [a,b] 切成 n 份，每份用梯形近似面积
pub fn trapezoid<F: Fn(f64) -> f64>(f: F, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = 0.5 * (f(a) + f(b)); // 两端点各算一半
    for i in 1..n {
        sum += f(a + h * i as f64);
    }
    sum * h
}

// 辛普森法：每两个子区间用抛物线近似，精度更高（需要 n 为偶数）
pub fn simpson<F: Fn(f64) -> f64>(f: F, a: f64, b: f64, n: usize) -> f64 {
    assert!(n % 2 == 0, "辛普森法要求偶数分段");
    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);
    for i in 1..n {
        let x = a + h * i as f64;
        let weight = if i % 2 == 0 { 2.0 } else { 4.0 }; // 偶数点 ×2，奇数点 ×4
        sum += weight * f(x);
    }
    sum * h / 3.0
}

// ============================================
// 二、SVG 绘图
// ============================================

// 把函数曲线画成 SVG（可缩放矢量图），浏览器直接打开
// 结构：SVG 头 + 坐标轴 + 曲线折线 + 网格
pub fn plot_svg<F: Fn(f64) -> f64>(
    f: F,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    path: &str,
) -> Result<(), String> {
    let width = 800.0;
    let height = 500.0;
    let margin = 40.0; // 边距
    let samples = 400; // 采样点数

    // 坐标映射：数学坐标 → SVG 像素坐标
    let map_x = |x: f64| margin + (x - xmin) / (xmax - xmin) * (width - 2.0 * margin);
    let map_y = |y: f64| height - margin - (y - ymin) / (ymax - ymin) * (height - 2.0 * margin);

    // 生成曲线点集
    let mut points = String::new();
    for i in 0..=samples {
        let x = xmin + (xmax - xmin) * i as f64 / samples as f64;
        let y = f(x);
        if y.is_finite() && y >= ymin && y <= ymax {
            points.push_str(&format!("{:.1},{:.1} ", map_x(x), map_y(y)));
        } else {
            // 断开不连续点：插入无效点（SVG 用分号跳过）
            points.push_str("; ");
        }
    }

    // 网格线（轻量实现：几条参考线）
    let mut grid = String::new();
    for (x, label) in [
        (xmin, format!("{:.1}", xmin)),
        (xmax, format!("{:.1}", xmax)),
    ] {
        let px = map_x(x);
        grid.push_str(&format!(
            r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#ddd" stroke-width="1"/>"##,
            px, margin, px, height - margin
        ));
        grid.push_str(&format!(
            r##"<text x="{}" y="{}" font-size="12" fill="#999">{}</text>"##,
            px - 10.0,
            height - margin + 15.0,
            label
        ));
    }
    // 水平零线
    let zero_y = map_y(0.0);
    grid.push_str(&format!(
        r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#bbb" stroke-width="1"/>"##,
        margin, zero_y, width - margin, zero_y
    ));

    // 坐标轴
    let axes = format!(
        r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#333" stroke-width="2"/>
           <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#333" stroke-width="2"/>"##,
        margin, height - margin, width - margin, height - margin, // x 轴
        margin, margin, margin, height - margin, // y 轴
    );

    // 曲线（红色）
    let curve = format!(
        r##"<polyline points="{}" fill="none" stroke="#e74c3c" stroke-width="2.5"/>"##,
        points
    );

    // 标题
    let title = format!(
        r##"<text x="{}" y="{}" font-size="18" fill="#333" font-weight="bold">函数曲线（Rust 生成）</text>"##,
        width / 2.0 - 100.0,
        margin / 2.0 + 10.0
    );

    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">
{}{}{}{}{}
</svg>"##,
        width, height, title, grid, axes, curve, ""
    );

    fs::write(path, svg).map_err(|e| format!("写文件失败: {}", e))
}

// ============================================
// 三、演示用函数
// ============================================
fn f_sin(x: f64) -> f64 {
    x.sin()
}
fn f_quadratic(x: f64) -> f64 {
    x * x
}
fn f_exp(x: f64) -> f64 {
    x.exp()
}

fn main() {
    println!("════════════════════════════════════════");
    println!("  数值积分演示（对比不同精度的数值方法）");
    println!("════════════════════════════════════════");

    // ∫₀¹ x² dx = 1/3 ≈ 0.3333333
    let exact = 1.0 / 3.0;
    println!("\n∫₀¹ x² dx = 1/3 ≈ {:.10}", exact);
    for &n in &[4usize, 10, 100, 1000] {
        let t = trapezoid(f_quadratic, 0.0, 1.0, n);
        let s = simpson(f_quadratic, 0.0, 1.0, n);
        println!(
            "  n={:<4} 梯形: {:.10} (误差 {:.2e})   辛普森: {:.10} (误差 {:.2e})",
            n,
            t,
            (t - exact).abs(),
            s,
            (s - exact).abs()
        );
    }

    // ∫₀^π sin(x) dx = 2
    let exact = 2.0;
    println!("\n∫₀^π sin(x) dx = 2");
    for &n in &[10usize, 100] {
        let t = trapezoid(f_sin, 0.0, PI, n);
        let s = simpson(f_sin, 0.0, PI, n);
        println!(
            "  n={:<4} 梯形: {:.10} (误差 {:.2e})   辛普森: {:.10} (误差 {:.2e})",
            n,
            t,
            (t - exact).abs(),
            s,
            (s - exact).abs()
        );
    }

    // ∫₀¹ eˣ dx = e - 1 ≈ 1.7182818
    let exact = std::f64::consts::E - 1.0;
    println!("\n∫₀¹ eˣ dx = e-1 ≈ {:.10}", exact);
    let s = simpson(f_exp, 0.0, 1.0, 100);
    println!("  n=100 辛普森: {:.10} (误差 {:.2e})", s, (s - exact).abs());

    println!("\n════════════════════════════════════════");
    println!("  SVG 绘图");
    println!("════════════════════════════════════════");

    // 画 sin(x) 曲线
    plot_svg(f_sin, -3.0 * PI, 3.0 * PI, -1.5, 1.5, "sin_curve.svg").unwrap();
    println!("已生成 sin_curve.svg（浏览器打开）");

    // 画 sinc(x) = sin(x)/x（信号处理经典函数）
    let sinc = |x: f64| {
        if x == 0.0 {
            1.0
        } else {
            x.sin() / x
        }
    };
    plot_svg(sinc, -4.0 * PI, 4.0 * PI, -0.5, 1.2, "sinc_curve.svg").unwrap();
    println!("已生成 sinc_curve.svg");

    // 画高斯函数（正态分布）
    let gaussian = |x: f64| (-x * x).exp();
    plot_svg(gaussian, -3.0, 3.0, -0.2, 1.1, "gaussian_curve.svg").unwrap();
    println!("已生成 gaussian_curve.svg");
}

// ============================================
// 四、单元测试
// ============================================
#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-6;

    #[test]
    fn test_trapezoid_quadratic() {
        // ∫₀¹ x² dx = 1/3
        let result = trapezoid(f_quadratic, 0.0, 1.0, 1000);
        assert!((result - 1.0 / 3.0).abs() < EPS);
    }

    #[test]
    fn test_simpson_quadratic() {
        // 辛普森对二次函数是【精确】的（抛物线恰好拟合二次函数）！
        let result = simpson(f_quadratic, 0.0, 1.0, 2);
        assert!((result - 1.0 / 3.0).abs() < 1e-12);
    }

    #[test]
    fn test_simpson_sin() {
        // ∫₀^π sin(x) dx = 2
        let result = simpson(f_sin, 0.0, PI, 100);
        assert!((result - 2.0).abs() < EPS);
    }

    #[test]
    fn test_simpson_requires_even() {
        // 奇数分段必须 panic
        let result = std::panic::catch_unwind(|| simpson(f_sin, 0.0, PI, 3));
        assert!(result.is_err());
    }

    #[test]
    fn test_closures_as_functions() {
        // 闭包和函数都能传给积分器（都实现 Fn(f64) -> f64）
        let linear = |x: f64| 2.0 * x + 1.0;
        let result = simpson(linear, 0.0, 1.0, 2);
        assert!((result - 2.0).abs() < 1e-12); // ∫₀¹ (2x+1) dx = 2
    }

    #[test]
    fn test_plot_generates_file() {
        // 画图要能生成合法文件
        let path = "/tmp/rust_plot_test.svg";
        plot_svg(f_sin, -PI, PI, -1.0, 1.0, path).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(content.starts_with("<svg"));
        assert!(content.contains("polyline"));
        let _ = fs::remove_file(path);
    }
}
