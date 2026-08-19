// ============================================
// 第 25 课：实战项目三 —— 多线程文件统计
// ============================================
// 统计目录（含子目录）下所有文本文件的行数/单词数/字节数，
// 并对比【单线程】和【多线程】两种实现的耗时。
//
// 复习：第 19 课并发（thread + mpsc）、第 10 课集合、第 17 课迭代器、
//      第 6 课借用（chunks 切分）、错误处理。
//
// 用法：cargo run -- [目录] [线程数]
//   cargo run -- sample_data        # 默认 4 线程
//   cargo run -- sample_data 8      # 指定 8 线程

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

// ---------- 1. 数据结构 ----------
#[derive(Debug, Clone)]
struct FileStats {
    path: PathBuf,
    lines: usize,
    words: usize,
    bytes: usize,
}

// ---------- 2. 单文件统计 ----------
fn count_file(path: &Path) -> Result<FileStats, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("{}: {}", path.display(), e))?;
    Ok(FileStats {
        path: path.to_path_buf(),
        lines: content.lines().count(),          // 迭代器数行
        words: content.split_whitespace().count(), // 数单词
        bytes: content.len(),                    // UTF-8 字节数
    })
}

// ---------- 3. 递归收集目录下所有文件 ----------
fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, out)?; // 递归子目录
        } else {
            out.push(path);
        }
    }
    Ok(())
}

// ---------- 4. 单线程版本 ----------
fn run_single_thread(files: &[PathBuf]) -> (Vec<FileStats>, usize) {
    let mut stats = Vec::new();
    let mut errors = 0;
    for f in files {
        match count_file(f) {
            Ok(s) => stats.push(s),
            Err(_) => errors += 1,
        }
    }
    (stats, errors)
}

// ---------- 5. 多线程版本（map-reduce 模式）----------
// 分块 → 每线程一块（map）→ 通道汇总（reduce）
fn run_multi_thread(files: &[PathBuf], num_threads: usize) -> (Vec<FileStats>, usize) {
    let (tx, rx) = mpsc::channel();

    // 把文件列表切成 num_threads 块
    let chunk_size = files.len().div_ceil(num_threads).max(1);

    let mut handles = vec![];
    for chunk in files.chunks(chunk_size) {
        let chunk = chunk.to_vec(); // 每线程拥有自己的文件列表
        let tx = tx.clone();        // 每线程一个发送端
        handles.push(thread::spawn(move || {
            let mut local = Vec::new();
            for f in &chunk {
                if let Ok(s) = count_file(f) {
                    local.push(s);
                }
            }
            let _ = tx.send(local); // 发送本线程结果
        }));
    }
    drop(tx); // 主线程关闭自己的发送端，rx 才能结束

    // reduce：收集所有线程的结果
    let mut stats = Vec::new();
    for received in rx {
        stats.extend(received);
    }
    for h in handles {
        let _ = h.join();
    }

    let errors = files.len() - stats.len(); // 统计失败的文件数
    (stats, errors)
}

// ---------- 6. 汇总输出 ----------
fn print_summary(stats: &[FileStats], errors: usize, elapsed: std::time::Duration) {
    let total_lines: usize = stats.iter().map(|s| s.lines).sum();
    let total_words: usize = stats.iter().map(|s| s.words).sum();
    let total_bytes: usize = stats.iter().map(|s| s.bytes).sum();

    println!("文件数: {} (失败 {})", stats.len(), errors);
    println!("总行数: {}", total_lines);
    println!("总单词: {}", total_words);
    println!("总字节: {}", total_bytes);
    println!("耗时:   {:.2} ms", elapsed.as_secs_f64() * 1000.0);

    // 最大的几个文件
    println!("\n最大 3 个文件（按字节）:");
    let mut sorted = stats.to_vec();
    sorted.sort_by(|a, b| b.bytes.cmp(&a.bytes));
    for s in sorted.iter().take(3) {
        println!("  {:>8} B  {:>6} 行  {}", s.bytes, s.lines, s.path.display());
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = args.get(1).map(|s| s.as_str()).unwrap_or("sample_data");
    let num_threads: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(4);

    // 收集文件列表
    let mut files = Vec::new();
    if let Err(e) = collect_files(Path::new(dir), &mut files) {
        eprintln!("无法读取目录 {}: {}", dir, e);
        std::process::exit(1);
    }
    files.sort(); // 保证输出顺序稳定
    println!("目录 {} 下找到 {} 个文件，使用 {} 线程\n", dir, files.len(), num_threads);

    if files.is_empty() {
        println!("没有文件可统计");
        return;
    }

    // ---------- 单线程计时 ----------
    let start = Instant::now();
    let (single_stats, single_errors) = run_single_thread(&files);
    let single_time = start.elapsed();
    println!("===== 单线程 =====");
    print_summary(&single_stats, single_errors, single_time);

    // ---------- 多线程计时 ----------
    let start = Instant::now();
    let (multi_stats, multi_errors) = run_multi_thread(&files, num_threads);
    let multi_time = start.elapsed();
    println!("\n===== 多线程 ({} 线程) =====", num_threads);
    print_summary(&multi_stats, multi_errors, multi_time);

    // ---------- 结果一致性 + 加速比 ----------
    println!("\n===== 对比 =====");
    let single_lines: usize = single_stats.iter().map(|s| s.lines).sum();
    let multi_lines: usize = multi_stats.iter().map(|s| s.lines).sum();
    println!("结果一致: {} (单) vs {} (多)", single_lines, multi_lines);
    if multi_time < single_time && single_time.as_nanos() > 0 {
        let speedup = single_time.as_secs_f64() / multi_time.as_secs_f64();
        println!("加速比: {:.2}x", speedup);
    } else {
        println!("加速比: 文件太少/太小，多线程无优势（线程开销 > 计算量）");
    }
    println!("多线程意义：I/O 密集或大文件时收益明显；本 demo 的样本量小，主要演示机制");
}

// ---------- 7. 单元测试 ----------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_file() {
        // 用临时文件验证统计逻辑
        // 注意：split_whitespace 按【空白】分词，中文句子若无空格算 1 个词
        let path = "/tmp/file_stats_test.txt";
        fs::write(path, "hello world\nfoo bar baz\n").unwrap();
        let stats = count_file(Path::new(path)).unwrap();
        assert_eq!(stats.lines, 2);
        assert_eq!(stats.words, 5); // hello world foo bar baz = 5 个词
        assert_eq!(stats.bytes, "hello world\nfoo bar baz\n".len());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_collect_files_recursive() {
        // 临时目录树：根目录 2 个文件 + 子目录 1 个文件
        let root = "/tmp/file_stats_dir";
        let sub = format!("{}/sub", root);
        let _ = fs::create_dir_all(&sub);
        fs::write(format!("{}/a.txt", root), "a").unwrap();
        fs::write(format!("{}/b.txt", root), "b").unwrap();
        fs::write(format!("{}/c.txt", sub), "c").unwrap();

        let mut files = Vec::new();
        collect_files(Path::new(root), &mut files).unwrap();
        assert_eq!(files.len(), 3); // 递归找到子目录文件

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn test_multi_matches_single() {
        // 核心：多线程结果必须和单线程一致
        let root = "/tmp/file_stats_compare";
        let _ = fs::create_dir_all(root);
        for i in 0..8 {
            let content = format!("line{}\n", i).repeat(100);
            fs::write(format!("{}/f{}.txt", root, i), content).unwrap();
        }

        let mut files = Vec::new();
        collect_files(Path::new(root), &mut files).unwrap();

        let (single, _) = run_single_thread(&files);
        let (multi, _) = run_multi_thread(&files, 4);

        // 排序后逐条对比
        let mut single = single;
        single.sort_by(|a, b| a.path.cmp(&b.path));
        let mut multi = multi;
        multi.sort_by(|a, b| a.path.cmp(&b.path));

        assert_eq!(single.len(), multi.len());
        for (s, m) in single.iter().zip(multi.iter()) {
            assert_eq!(s.lines, m.lines);
            assert_eq!(s.words, m.words);
            assert_eq!(s.bytes, m.bytes);
        }
        let _ = fs::remove_dir_all(root);
    }
}
