// ============================================
// 第 28 课：实战项目六 —— 文本分析工具
// ============================================
// 分析文本文件：行数/词数/字符数、词频 Top-N、平均词长、最长词。
// 经典 wc 命令的增强版。
//
// 用法：
//   cargo run -- 文件.txt        # 分析文件
//   cargo run                    # 从标准输入读取（Ctrl+D 结束）
//
// 复习：HashMap 词频统计、迭代器链、sort、Result、单元测试

use std::collections::HashMap;
use std::io::Read;

// ---------- 1. 分析结果 ----------
#[derive(Debug, Default, PartialEq)]
pub struct TextStats {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,      // Unicode 字符数
    pub bytes: usize,      // UTF-8 字节数
    pub unique_words: usize,
    pub avg_word_len: f64, // 平均词长（按字符）
    pub longest_words: Vec<String>,
    pub top_words: Vec<(String, usize)>, // (单词, 出现次数)
}

// ---------- 2. 文本标准化：小写 + 去标点 ----------
// "Hello," → "hello"；"Rust!" → "rust"；"Don't" → "don't"（撇号保留）
fn normalize(word: &str) -> String {
    word.chars()
        .filter(|c| c.is_alphabetic() || *c == '\'') // 保留字母和撇号
        .flat_map(|c| c.to_lowercase())              // 转小写（支持非 ASCII）
        .collect()
}

// ---------- 3. 核心分析函数 ----------
pub fn analyze(text: &str) -> TextStats {
    // 一遍遍历完成：分词、标准化、词频统计、累计词长
    let mut words: Vec<String> = Vec::new();
    let mut freq: HashMap<String, usize> = HashMap::new();
    let mut total_len = 0usize;

    for raw in text.split_whitespace() {
        let word = normalize(raw);
        if word.is_empty() {
            continue; // 纯标点不算词
        }
        total_len += word.chars().count();
        words.push(word.clone());
        *freq.entry(word).or_insert(0) += 1; // entry API（第 10 课）
    }

    let word_count = words.len();
    let unique_words = freq.len();

    // 词频 → 排序取 Top 10（次数降序，同次数按字母序）
    let mut freq_vec: Vec<(String, usize)> = freq.into_iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let top_words = freq_vec.into_iter().take(10).collect();

    // 最长词：去重后按长度降序取前 5
    let mut unique_list = words.clone();
    unique_list.sort_by_key(|w| std::cmp::Reverse(w.chars().count()));
    unique_list.dedup();
    let longest_words = unique_list.into_iter().take(5).collect();

    TextStats {
        lines: text.lines().count(),
        words: word_count,
        chars: text.chars().count(),
        bytes: text.len(),
        unique_words,
        avg_word_len: if word_count > 0 {
            total_len as f64 / word_count as f64
        } else {
            0.0
        },
        longest_words,
        top_words,
    }
}

// ---------- 4. 打印报告 ----------
pub fn print_report(stats: &TextStats, source: &str) {
    println!("═══════════════════════════════════");
    println!("📄 文本分析报告 —— {}", source);
    println!("═══════════════════════════════════");
    println!("行数:      {}", stats.lines);
    println!("词数:      {}", stats.words);
    println!("字符数:    {}（{} 字节）", stats.chars, stats.bytes);
    println!("去重词数:  {}", stats.unique_words);
    println!("平均词长:  {:.2} 字符", stats.avg_word_len);

    println!("\n🏆 高频词 Top 10:");
    for (i, (word, count)) in stats.top_words.iter().enumerate() {
        println!("  {:>2}. {:<12} × {}", i + 1, word, count);
    }

    println!("\n📏 最长词 Top 5:");
    for w in &stats.longest_words {
        println!("  {}（{} 字符）", w, w.chars().count());
    }
    println!("═══════════════════════════════════");
}

// ---------- 5. main ----------
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (content, source) = if args.len() > 1 {
        // 文件模式
        let path = &args[1];
        match std::fs::read_to_string(path) {
            Ok(c) => (c, path.clone()),
            Err(e) => {
                eprintln!("无法读取 {}: {}", path, e);
                std::process::exit(1);
            }
        }
    } else {
        // 标准输入模式
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();
        (buf, String::from("标准输入"))
    };

    let stats = analyze(&content);
    print_report(&stats, &source);
}

// ---------- 6. 单元测试 ----------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        assert_eq!(normalize("Hello,"), "hello");
        assert_eq!(normalize("Rust!"), "rust");
        assert_eq!(normalize("  "), "");
        assert_eq!(normalize("Don't"), "don't"); // 撇号保留
    }

    #[test]
    fn test_basic_stats() {
        let stats = analyze("Hello world hello Rust rust Rust");
        assert_eq!(stats.words, 6);
        assert_eq!(stats.unique_words, 3); // hello, world, rust
        assert_eq!(stats.lines, 1);
        // 大小写归一化后：rust 3 次，hello 2 次，world 1 次
        assert_eq!(stats.top_words[0], ("rust".to_string(), 3));
        assert_eq!(stats.top_words[1], ("hello".to_string(), 2));
    }

    #[test]
    fn test_punctuation_handling() {
        let stats = analyze("Hello, world! This is Rust. Rust is fun, is it?");
        // 标点不影响词频
        assert_eq!(stats.top_words[0], ("is".to_string(), 3));
        assert_eq!(stats.unique_words, 7);
    }

    #[test]
    fn test_empty_and_whitespace() {
        let stats = analyze("");
        assert_eq!(stats.words, 0);
        assert_eq!(stats.lines, 0);
        assert_eq!(stats.avg_word_len, 0.0);

        let stats2 = analyze("   \n  \n ");
        assert_eq!(stats2.words, 0);
        assert_eq!(stats2.lines, 3);
    }

    #[test]
    fn test_longest_words() {
        let stats = analyze("a bb ccc dddd eeeee ffffff");
        assert_eq!(stats.longest_words[0], "ffffff");
        assert_eq!(stats.longest_words.len(), 5);
    }

    #[test]
    fn test_unicode() {
        let stats = analyze("你好世界 你好 Rust 编程 编程 编程");
        assert_eq!(stats.words, 6); // 你好世界 你好 Rust 编程 编程 编程 = 6 个词
        assert_eq!(stats.top_words[0], ("编程".to_string(), 3));
        // 中文每字符 3 字节
        assert_eq!(stats.bytes, "你好世界 你好 Rust 编程 编程 编程".len());
        assert_eq!(stats.chars, "你好世界 你好 Rust 编程 编程 编程".chars().count());
    }
}
