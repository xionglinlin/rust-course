// ============================================
// 第 7 课：字符串 String 与 &str
// ============================================

// Rust 的字符串有两套：
// 1. String：可增长、可修改、拥有所有权的堆字符串
// 2. &str：字符串切片，不可变借用，最常用的是字符串字面量
//
// 记忆：String 是"容器"，&str 是"视图"（切片）
// 类比：String ≈ 拥有房屋的所有权；&str ≈ 进去参观的邀请函

fn main() {
    // ---------- 1. 创建 String ----------
    let s1 = String::new();                    // 空字符串
    let s2 = String::from("hello");            // 从字面量创建
    let s3 = "world".to_string();              // 另一种方式
    let mut s4 = String::from("你好");
    println!("s1='{}', s2='{}', s3='{}', s4='{}'", s1, s2, s3, s4);

    // ---------- 2. 修改 String ----------
    s4.push_str(", Rust!"); // 追加字符串
    s4.push('😀');           // 追加单个字符
    println!("追加后: {}", s4);

    // 拼接：+ 运算符（注意：左边必须是 String，右边 &str，结果是新 String）
    let hello = String::from("Hello, ");
    let world = String::from("World!");
    let msg = hello + &world; // hello 被移动进 +，之后 hello 不能再用
    println!("拼接: {}", msg);

    // 更优雅的拼接：format! 宏（不移动任何变量）
    let a = String::from("红");
    let b = String::from("绿");
    let c = format!("{}-{}-蓝", a, b);
    println!("format!: {}", c);
    println!("a 和 b 都还能用: {} {}", a, b); // ✅

    // ---------- 3. &str 切片 ----------
    // 字符串字面量的类型是 &str
    let literal: &str = "这是一个字符串字面量";
    println!("字面量: {}", literal);

    // &String 可以自动"解引用强制转换"为 &str（Deref coercion）
    let s = String::from("自动转换");
    print_str(&s); // 传 &String，函数收 &str，自动转换 ✅
    print_str("直接传字面量"); // ✅

    // 切片：&s[start..end] 按字节索引（不是字符！）
    let text = String::from("hello world");
    let first = &text[0..5];  // "hello"
    let last = &text[6..11];  // "world"
    println!("切片: '{}' + '{}'", first, last);

    // ⚠️ 中文按 UTF-8 编码占 3 字节，按字节切片可能切到字符中间 → panic
    // let cn = String::from("中文测试");
    // println!("{}", &cn[0..2]); // ❌ 取消注释：byte index 2 is not a char boundary
    // 正确做法：先找字符边界，或用 chars()

    // ---------- 4. 遍历 ----------
    // 按字节遍历
    for b in "abc".bytes() {
        print!("{} ", b);
    }
    println!();

    // 按字符（Unicode 标量）遍历——推荐
    for c in "中文😀".chars() {
        print!("[{}] ", c);
    }
    println!();

    // ---------- 5. 常用方法 ----------
    let s = String::from("  Hello, Rust!  ");
    println!("长度 len(): {}", s.len());              // 字节数
    println!("去掉首尾空白: '{}'", s.trim());
    println!("转大写: {}", s.to_uppercase());
    println!("转小写: {}", s.trim().to_lowercase());
    println!("包含 'Rust'? {}", s.contains("Rust"));
    println!("替换: {}", s.replace("Rust", "World"));
    println!("以 Hello 开头? {}", s.trim_start().starts_with("Hello"));

    // 分割
    let csv = "apple,banana,orange";
    for fruit in csv.split(',') {
        print!("{} ", fruit);
    }
    println!();

    // ---------- 6. 字符串与数字互转 ----------
    let num_str = "42".to_string();
    let num: i32 = num_str.parse().expect("解析失败");
    println!("字符串->数字: {} + 1 = {}", num, num + 1);

    let back = num.to_string();
    println!("数字->字符串: {}", back);

    // ---------- 7. 上一课遗留问题的解法：first_word ----------
    let sentence = String::from("Hello world");
    let w = first_word(&sentence);
    println!("第一个单词是: '{}'", w);
    // w 是 &str，借用 sentence，sentence 依然可用
    println!("句子还在: {}", sentence);

    // 第 6 课练习 3 的报错原因：返回的 &str 引用了函数的局部变量
    // 正确版本见下方函数定义——返回的切片借用的是参数，不是局部变量
}

// 接受 &str（比 &String 更通用：&String 和字面量都能传）
fn print_str(s: &str) {
    println!("print_str 收到: {}", s);
}

// 返回第一个单词的切片（借用参数，不拥有）
// 注意：返回类型省略了生命周期标注（第 12 课会讲），这里编译器自动推断
fn first_word(s: &str) -> &str {
    // 找第一个空格
    match s.find(' ') {
        Some(pos) => &s[..pos], // 切到空格前
        None => s,              // 没有空格，整个都是第一个单词
    }
}
