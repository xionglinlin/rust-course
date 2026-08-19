// ============================================
// 第 13 课：trait —— Rust 的"接口"（能力抽象）
// ============================================

// trait 定义"一组行为/能力"。类型实现 trait = 拥有这些能力。
// 类比：Java/C# 的 interface，但 Rust 的 trait 更强大：
//   - 可以为任何类型实现（包括标准库类型）
//   - 有默认方法实现
//   - 泛型约束（trait bound）让代码同时获得多态和零开销

use std::fmt::Display;

// ---------- 1. 定义 trait ----------
trait Summary {
    // 只有签名，实现者必须提供
    fn summarize(&self) -> String;

    // 有默认实现的方法：实现者可以覆盖，也可以直接用
    fn author(&self) -> String {
        String::from("匿名作者")
    }

    // 默认实现里可以调用 trait 的其它方法
    fn summary_with_author(&self) -> String {
        format!("{} —— {}", self.summarize(), self.author())
    }
}

// ---------- 2. 为类型实现 trait ----------
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} ({}) —— {}（内容：{}）", self.headline, self.location, self.author, self.content)
    }
    // author() 用默认实现（匿名作者）
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        let tag = if self.retweet { " 🔁转推" } else { "" };
        let reply_tag = if self.reply { " 💬回复" } else { "" };
        format!("{} 说: {}{}{}", self.username, self.content, tag, reply_tag)
    }

    // 覆盖默认实现
    fn author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    // ---------- 3. 使用 trait 方法 ----------
    let article = NewsArticle {
        headline: String::from("Rust 1.96 发布了！"),
        location: String::from("北京"),
        author: String::from("小张"),
        content: String::from("..."),
    };
    let tweet = Tweet {
        username: String::from("rust_fan"),
        content: String::from("今天学完了 trait！"),
        reply: false,
        retweet: true,
    };

    println!("新闻: {}", article.summarize());
    println!("推文: {}", tweet.summarize());
    println!("新闻作者: {}", article.author());       // 默认实现
    println!("推文作者: {}", tweet.author());         // 覆盖的实现
    println!("{}", article.summary_with_author());    // 默认方法调 summarize + author

    // ---------- 4. trait 作为参数：impl Trait 语法 ----------
    // 接受"任何实现了 Summary 的类型"
    print_summary(&article);
    print_summary(&tweet);

    // ---------- 5. trait bound：泛型约束 ----------
    // 上一课的 <T: PartialOrd> 就是这个东西
    println!("{}", notify(&article));
    println!("{}", notify(&tweet));
    // where 子句写法（约束多时更清晰）
    println!("{}", notify2(&article));

    // ---------- 6. 返回实现了 trait 的类型 ----------
    let my_summary = returns_summarizable();
    println!("返回的类型: {}", my_summary.summarize());

    // ---------- 7. 标准库 trait 实战：Display ----------
    // 让自定义类型支持 {} 打印
    let p = Person {
        name: String::from("小明"),
        age: 25,
    };
    println!("Person 用 Display 打印: {}", p);

    // ---------- 8. 多个 trait bound ----------
    // T 既要能显示，又要能比较
    let biggest = max_displayable(3, 7);
    println!("比较大的: {}", biggest);

    // ---------- 9. derive：自动实现 ----------
    // #[derive(...)] 是编译器自动生成实现，等价于手写 impl
    // 常见的：Debug、Clone、Copy、PartialEq、Eq、Hash
    let c1 = Color { r: 255, g: 0, b: 0 };
    let c2 = Color { r: 255, g: 0, b: 0 };
    println!("c1 = {:?}", c1);                       // Debug
    let c3 = c1.clone();                             // Clone
    println!("c1 == c2 ? {}", c1 == c2);             // PartialEq
    println!("克隆的 c3 = {:?}", c3);
}

// ---------- 4. impl Trait 参数语法 ----------
fn print_summary(item: &impl Summary) {
    println!("简讯: {}", item.summarize());
}

// ---------- 5. trait bound 泛型（等价写法）----------
// 上面是语法糖，这是完整写法。多参数时用 where 更清晰
fn notify<T: Summary>(item: &T) -> String {
    format!("通知: {}", item.summarize())
}

// where 子句：约束多时更易读
fn notify2<T>(item: &T) -> String
where
    T: Summary,
{
    format!("通知: {}", item.summarize())
}

// ---------- 6. 返回 trait：只能返回单一具体类型 ----------
fn returns_summarizable() -> impl Summary {
    // 注意：不能用 if 返回不同类型（NewsArticle 或 Tweet）——需要"特征对象"，
    // 第 18 课智能指针讲 Box<dyn Trait> 时展开
    Tweet {
        username: String::from("temp"),
        content: String::from("返回类型是 impl Summary"),
        reply: false,
        retweet: false,
    }
}

// ---------- 7. 标准库 trait：Display ----------
struct Person {
    name: String,
    age: u32,
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}（{} 岁）", self.name, self.age)
    }
}

// ---------- 8. 多个 trait bound ----------
// T: Display + PartialOrd —— T 既能显示又能比较
fn max_displayable<T>(a: T, b: T) -> T
where
    T: Display + PartialOrd,
{
    if a > b { a } else { b }
}

// ---------- 9. derive 示例 ----------
#[derive(Debug, Clone, PartialEq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}
