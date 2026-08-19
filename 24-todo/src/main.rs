// ============================================
// 第 24 课：Todo 命令行工具（二进制 crate：命令行解析）
// ============================================
// main.rs 保持"薄"：只负责读命令行参数、调用库、展示结果。
// 业务逻辑全在 lib.rs，可独立测试。
//
// 用法：
//   cargo run -- add "学习 Rust"
//   cargo run -- list
//   cargo run -- done 1
//   cargo run -- remove 1
//   cargo run -- clear
//   cargo test          # 跑库的单元测试

use todo::{TodoItem, TodoStore};

const DATA_FILE: &str = "todo_data.txt";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // 打开存储（失败则退出）
    let mut store = match TodoStore::open(DATA_FILE) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("无法打开数据文件: {}", e);
            std::process::exit(1);
        }
    };

    // 没有参数 → 打印用法
    if args.len() < 2 {
        print_usage();
        return;
    }

    let cmd = args[1].as_str();
    let result = match cmd {
        "add" => {
            // 剩余参数全部拼成描述
            let desc = args[2..].join(" ");
            if desc.trim().is_empty() {
                println!("用法: todo add <描述>");
                Ok(None)
            } else {
                store.add(&desc).map(|item| Some(format!("已添加: #{} {}", item.id, item.description)))
            }
        }
        "list" => {
            let items = store.list().to_vec();
            if items.is_empty() {
                println!("暂无任务");
                Ok(None)
            } else {
                let mut out = String::new();
                for item in &items {
                    out.push_str(&format_item(item));
                    out.push('\n');
                }
                out.pop(); // 去掉末尾换行
                let (total, done) = store.stats();
                out.push_str(&format!("\n共 {} 项，已完成 {} 项", total, done));
                Ok(Some(out))
            }
        }
        "done" => {
            match args.get(2).and_then(|s| s.parse::<u32>().ok()) {
                Some(id) => store
                    .done(id)
                    .map(|opt| opt.map(|item| format!("完成: #{} {}", item.id, item.description))),
                None => {
                    println!("用法: todo done <id>");
                    Ok(None)
                }
            }
        }
        "remove" => {
            match args.get(2).and_then(|s| s.parse::<u32>().ok()) {
                Some(id) => store
                    .remove(id)
                    .map(|opt| opt.map(|item| format!("已删除: #{} {}", item.id, item.description))),
                None => {
                    println!("用法: todo remove <id>");
                    Ok(None)
                }
            }
        }
        "clear" => store
            .clear()
            .map(|count| Some(format!("已清空 {} 项", count))),
        _ => {
            print_usage();
            Ok(None)
        }
    };

    // 统一处理结果
    match result {
        Ok(Some(msg)) => println!("{}", msg),
        Ok(None) => {}
        Err(e) => eprintln!("错误: {}", e),
    }
}

// 展示单条任务
fn format_item(item: &TodoItem) -> String {
    let mark = if item.done { "[x]" } else { "[ ]" };
    format!("{} #{} {}", mark, item.id, item.description)
}

fn print_usage() {
    println!("Todo 命令行工具");
    println!("用法:");
    println!("  cargo run -- add <描述>    添加任务");
    println!("  cargo run -- list          列出任务");
    println!("  cargo run -- done <id>     标记完成");
    println!("  cargo run -- remove <id>   删除任务");
    println!("  cargo run -- clear         清空任务");
    println!("数据保存在 {}", DATA_FILE);
}
