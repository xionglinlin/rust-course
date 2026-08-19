// ============================================
// 第 15 课：模块与包管理 —— 二进制 crate（main.rs）
// ============================================

// main.rs 是二进制 crate，它使用本项目的库 crate（lib.rs）
// 库 crate 的名字 = Cargo.toml 里的包名（modules）

// ---------- 1. use：引入路径 ----------
// 完整路径写法：crate 名 :: 模块 :: 模块 :: 项
use modules::greeting::hello;
use modules::math::{add, subtract};

// use 的其它形式：
// use modules::math::advanced as adv;   // as 起别名
// use modules::*;                       // 通配导入（谨慎用）
// use modules::math::advanced::{power, sqrt}; // 多重导入

fn main() {
    // ---------- 2. 调用库里的公开项 ----------
    println!("{}", hello("小明"));

    println!("3 + 5 = {}", add(3, 5));
    println!("10 - 4 = {}", subtract(10, 4));

    // 嵌套路径调用：没有 use 时用完整路径
    println!("2^10 = {}", modules::math::advanced::power(2.0, 10.0));
    println!("√16 = {}", modules::math::advanced::sqrt(16.0));

    println!("库版本: {}", modules::version());

    // ---------- 3. 可见性演示 ----------
    println!("{}", modules::greeting::bye("小红")); // pub：二进制能用
    println!("{}", modules::greeting_hint());        // pub(crate) 经库封装后暴露
    println!("{}", modules::greeting::secret_message()); // 私有函数经公开方法间接暴露
    // 私有项：模块外不可见（取消注释会编译错误）
    // println!("{}", modules::greeting::secret()); // ❌ E0603: 私有
    // println!("{}", modules::greeting::internal_hint()); // ❌ pub(crate) 跨 crate 不可见
    // println!("{}", modules::math::sub(1, 2));    // ❌ 私有

    // ---------- 4. 本地模块：main.rs 内也能定义模块 ----------
    mod utils {
        pub fn double(x: i32) -> i32 {
            x * 2
        }
    }
    println!("double(21) = {}", utils::double(21));

    // ---------- 5. 路径形式小结 ----------
    // 绝对路径：crate:: 开头（crate 根）
    // 相对路径：self::（当前模块）、super::（父模块）
    // 跨 crate：crate 名:: 开头（如 modules::）
    println!("路径体系: crate:: 绝对路径 / self:: super:: 相对路径 ✅");
}
