# Rust 入门到精通 —— 课程目录

> 环境：rustc 1.96.0 / cargo 1.96.0
> 每个 demo 都是独立的 cargo 项目，进入目录后 `cargo run` 即可运行。
> **全 30 课完成 ✅**（2026-08 全部编译运行验证通过）

## 第一阶段：入门（基础语法）

| 课程 | 项目目录 | 核心内容 | 状态 |
|------|----------|----------|------|
| 第 1 课 | `01-hello-world` | 程序结构、main 函数、println! 宏、注释 | ✅ 完成 |
| 第 2 课 | `02-variables-and-types` | 不可变/可变变量、常量、遮蔽、整数/浮点/布尔/字符、元组、数组 | ✅ 完成 |
| 第 3 课 | `03-functions` | 函数定义、参数、返回值、语句 vs 表达式、提前返回 | ✅ 完成 |
| 第 4 课 | `04-control-flow` | 控制流：if/else、loop、while、for、循环标签、continue/break | ✅ 完成 |
| 第 5 课 | `05-ownership` | 所有权：栈 vs 堆、移动 Move、Clone、Copy 类型、函数传参/返回值转移所有权 | ✅ 完成 |
| 第 6 课 | `06-borrowing` | 借用与引用：& 不可变借用、&mut 可变借用、借用规则、悬垂引用拦截、解引用 | ✅ 完成 |
| 第 7 课 | `07-strings` | 字符串：String vs &str、UTF-8、切片、遍历、常用方法、数字互转、first_word | ✅ 完成 |
| 第 8 课 | `08-structs` | 结构体：具名字段/元组/单元结构体、更新语法、impl 方法、关联函数 | ✅ 完成 |
| 第 9 课 | `09-enums-match` | 枚举与模式匹配：带数据枚举、Option、match 穷尽性、if let、解构、@ 绑定 | ✅ 完成 |
| 第 10 课 | `10-collections` | 集合：Vec、HashMap、HashSet、entry API、集合运算、去重、单词统计 | ✅ 完成 |
| 第 11 课 | `11-error-handling` | 错误处理：panic、Result、unwrap/expect、? 运算符、map 组合子、自定义错误 | ✅ 完成 |

**✅ 入门阶段完成（11 课）！**

## 第二阶段：进阶

| 课程 | 项目目录 | 核心内容 | 状态 |
|------|----------|----------|------|
| 第 12 课 | `12-generics` | 泛型：泛型函数、泛型结构体/枚举、泛型方法、单态化零成本抽象 | ✅ 完成 |
| 第 13 课 | `13-traits` | trait：定义/实现、默认方法、impl Trait、trait bound、Display、derive | ✅ 完成 |
| 第 14 课 | `14-lifetimes` | 生命周期：'a 标注、省略规则、结构体中的生命周期、'static、E0597 验证 | ✅ 完成 |
| 第 15 课 | `15-modules` | 模块与包管理：lib.rs/main.rs、mod 树、use 路径、可见性 pub/pub(crate)、多文件项目 | ✅ 完成 |
| 第 16 课 | `16-closures` | 闭包：语法、捕获环境、Fn/FnMut/FnOnce、move、作为参数/返回值 | ✅ 完成 |
| 第 17 课 | `17-iterators` | 迭代器：iter/into_iter/iter_mut、map/filter/fold、链式流水线、自定义 Iterator | ✅ 完成 |
| 第 18 课 | `18-smart-pointers` | 智能指针：Box 递归类型、Rc 引用计数、RefCell 内部可变、Box<dyn Trait> | ✅ 完成 |
| 第 19 课 | `19-concurrency` | 并发：thread::spawn、move 闭包、mpsc 通道、Arc<Mutex> 共享状态、Send/Sync | ✅ 完成 |
| 第 20 课 | `20-async` | 异步：Future/poll、async fn 惰性、await、手写 mini executor、并发 Join2 | ✅ 完成 |
| 第 21 课 | `21-unsafe-ffi` | unsafe：裸指针、unsafe fn、static mut、FFI 调 C、安全封装、E0133 验证 | ✅ 完成 |
| 第 22 课 | `22-macros` | 宏：macro_rules! 模式匹配、重复段、递归 token 收缩、宏生成函数、过程宏概念 | ✅ 完成 |
| 第 23 课 | `23-calculator` | 实战项目：命令行计算器（词法分析、递归下降解析、8 个单元测试、REPL） | ✅ 完成 |
| 第 24 课 | `24-todo` | 实战项目：Todo 工具（lib+bin 结构、文件持久化、手写序列化、5 个测试） | ✅ 完成 |
| 第 25 课 | `25-file-stats` | 实战项目：多线程文件统计（递归目录、map-reduce、单/多线程对比、3 个测试） | ✅ 完成 |
| 第 26 课 | `26-json-parser` | 实战项目：迷你 JSON 解析器（递归枚举 Value、char 游标、转义序列、7 个测试） | ✅ 完成 |
| 第 27 课 | `27-web-server` | 实战项目：迷你 Web 服务器（TCP、手写线程池、Message 优雅关闭、curl 实测） | ✅ 完成 |
| 第 28 课 | `28-text-analysis` | 实战项目：文本分析工具（词频统计、归一化、文件/stdin 双模式、6 个测试） | ✅ 完成 |
| 第 29 课 | `29-scientific-plot` | 实战项目：科学计算绘图（数值积分、收敛性对比、SVG 绘图输出、6 个测试） | ✅ 完成 |
| 第 30 课 | `30-graduation` | 毕业项目：能力自检 10/10 通过 + 生态巡礼 + 学习路线图 | ✅ 完成 |

**🎓 全部 30 课完成！** 每课 demo 均编译运行验证通过。

## 第三阶段：进阶方向（毕业后）

- Web/异步生态：tokio、axum、actix-web（第 20 课 async 的实战延伸）
- 序列化：serde + serde_json（第 26 课 JSON 解析器的工业级版）
- CLI 框架：clap（第 23-24 课命令行工具的专业化）
- 并行计算：rayon（第 25 课 map-reduce 的一行版）
- 测试进阶：criterion（基准测试）、proptest（属性测试）
- 数据库：sqlx、diesel、rusqlite
- GUI：egui、iced、slint；游戏：bevy
- 嵌入式：embedded-hal、esp-rs；操作系统：Linux 内核、Redox

## 如何运行

```bash
cd 01-hello-world
cargo run
```

推荐练习方式：每课看完后，**自己动手改代码**（比如改数字、加打印、故意写错看报错），这是理解 Rust 编译器的关键。
