// ============================================
// 第 24 课：实战项目二 —— Todo 命令行工具（库 crate）
// ============================================
// 工程结构（第 15 课的实战版）：
//   lib.rs  → 业务逻辑（数据模型 + 存储 + 命令），可测试
//   main.rs → 命令行解析（薄壳，只管读参数调库）
//
// 知识点复习：
//   - struct + enum + impl          - Result + ? 错误传播
//   - 文件读写（std::fs）           - 迭代器/集合操作
//   - 手写序列化（无 serde 依赖）    - 单元测试

use std::fs;
use std::path::Path;

// ---------- 1. 数据模型 ----------
#[derive(Debug, Clone, PartialEq)]
pub struct TodoItem {
    pub id: u32,
    pub description: String,
    pub done: bool,
}

// 序列化格式（每行一条）：
//   id|描述|0/1
// 手写实现，不依赖 serde（生产项目用 serde_json + derive）
impl TodoItem {
    fn to_line(&self) -> String {
        format!("{}|{}|{}", self.id, self.description, if self.done { 1 } else { 0 })
    }

    fn from_line(line: &str) -> Result<TodoItem, String> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 3 {
            return Err(format!("格式错误: {}", line));
        }
        Ok(TodoItem {
            id: parts[0]
                .parse()
                .map_err(|_| format!("无效 id: {}", parts[0]))?,
            description: parts[1].to_string(),
            done: parts[2] == "1",
        })
    }
}

// ---------- 2. 存储层 ----------
pub struct TodoStore {
    items: Vec<TodoItem>,
    next_id: u32,
    path: String, // 数据文件路径
}

impl TodoStore {
    // 打开（或创建）存储
    pub fn open(path: &str) -> Result<TodoStore, String> {
        let mut store = TodoStore {
            items: Vec::new(),
            next_id: 1,
            path: path.to_string(),
        };

        if Path::new(path).exists() {
            let content = fs::read_to_string(path)
                .map_err(|e| format!("读取文件失败: {}", e))?;
            for line in content.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                store.items.push(TodoItem::from_line(line)?);
            }
            // 下一个 id = 现有最大 id + 1
            store.next_id = store.items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
        }
        Ok(store)
    }

    // 保存到文件
    fn save(&self) -> Result<(), String> {
        let content = self
            .items
            .iter()
            .map(|item| item.to_line())
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(&self.path, content).map_err(|e| format!("保存失败: {}", e))
    }

    // ---------- 命令实现 ----------
    pub fn add(&mut self, description: &str) -> Result<TodoItem, String> {
        let item = TodoItem {
            id: self.next_id,
            description: description.trim().to_string(),
            done: false,
        };
        self.next_id += 1;
        self.items.push(item.clone());
        self.save()?;
        Ok(item)
    }

    pub fn list(&self) -> &[TodoItem] {
        &self.items
    }

    // 标记完成，返回被修改的条目
    pub fn done(&mut self, id: u32) -> Result<Option<TodoItem>, String> {
        // ⚠️ 借用检查器教学时刻：不能用 iter_mut().find() 拿到 &mut 后调 self.save()
        // （可变借用还活着，不能再借用 self）。改用索引式访问：
        // 1. 先找位置（不可变借用，立即结束）
        let idx = self
            .items
            .iter()
            .position(|i| i.id == id)
            .ok_or_else(|| format!("没有 id={} 的任务", id))?;
        // 2. 通过索引修改（借用是临时的，表达式结束后释放）
        self.items[idx].done = true;
        let cloned = self.items[idx].clone();
        // 3. 此时没有活跃借用，可以安全调用 self.save()
        self.save()?;
        Ok(Some(cloned))
    }

    // 删除
    pub fn remove(&mut self, id: u32) -> Result<Option<TodoItem>, String> {
        let idx = self
            .items
            .iter()
            .position(|i| i.id == id)
            .ok_or_else(|| format!("没有 id={} 的任务", id))?;
        let removed = self.items.remove(idx);
        self.save()?;
        Ok(Some(removed))
    }

    // 清空（保留文件）
    pub fn clear(&mut self) -> Result<usize, String> {
        let count = self.items.len();
        self.items.clear();
        self.save()?;
        Ok(count)
    }

    // 统计（展示用）
    pub fn stats(&self) -> (usize, usize) {
        let total = self.items.len();
        let done_count = self.items.iter().filter(|i| i.done).count();
        (total, done_count)
    }
}

// ---------- 3. 单元测试 ----------
#[cfg(test)]
mod tests {
    use super::*;

    // 每个测试用独立临时文件，避免互相干扰
    fn test_path(name: &str) -> String {
        format!("/tmp/todo_test_{}.txt", name)
    }

    #[test]
    fn test_add_and_list() {
        let path = test_path("add");
        let mut store = TodoStore::open(&path).unwrap();
        store.add("学习 Rust").unwrap();
        store.add("写 demo").unwrap();
        assert_eq!(store.list().len(), 2);
        assert_eq!(store.list()[0].description, "学习 Rust");
        assert_eq!(store.list()[0].id, 1);
        assert_eq!(store.list()[1].id, 2);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_done_and_stats() {
        let path = test_path("done");
        let mut store = TodoStore::open(&path).unwrap();
        store.add("任务A").unwrap();
        store.add("任务B").unwrap();
        store.done(1).unwrap();
        let (total, done) = store.stats();
        assert_eq!((total, done), (2, 1));
        assert!(store.list()[0].done);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_remove_and_missing() {
        let path = test_path("remove");
        let mut store = TodoStore::open(&path).unwrap();
        store.add("要被删的任务").unwrap();
        store.remove(1).unwrap();
        assert!(store.list().is_empty());
        // 删除不存在的 id → Err
        assert!(store.remove(99).is_err());
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_persistence() {
        // 关键测试：数据能保存并重新加载
        let path = test_path("persist");
        {
            let mut store = TodoStore::open(&path).unwrap();
            store.add("持久化测试").unwrap();
            store.done(1).unwrap();
        } // 离开作用域（保存已在命令中完成）
        {
            let store = TodoStore::open(&path).unwrap();
            assert_eq!(store.list().len(), 1);
            assert!(store.list()[0].done);
            // id 自增正确恢复：下一个 id 是 2
            assert_eq!(store.next_id, 2);
        }
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let item = TodoItem {
            id: 42,
            description: String::from("简单文本描述"),
            done: true,
        };
        let line = item.to_line();
        let parsed = TodoItem::from_line(&line).unwrap();
        assert_eq!(parsed, item);
        // ⚠️ 简单格式的局限：描述里不能含 '|'（会被当成字段分隔符）。
        // 生产项目用 serde_json 序列化，天然规避这类问题（转义由库处理）。
    }
}
