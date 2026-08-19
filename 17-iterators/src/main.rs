// ============================================
// 第 17 课：迭代器 Iterators 与组合子
// ============================================

// 迭代器 = 惰性的元素序列。真正的数据处理靠"组合子"（组合子 = 返回迭代器的方法）。
// 链式调用 map/filter/fold 等，把数据变换写成声明式流水线。
// 惰性：迭代器不消费元素，直到被"消费"（collect/sum/for 等）才执行。

fn main() {
    // ---------- 1. 三种迭代方式 ----------
    let v = vec![1, 2, 3, 4, 5];

    // iter()：借用元素（&i32），不移动
    let sum_ref: i32 = v.iter().sum();
    println!("iter() 借用求和: {}", sum_ref);

    // into_iter()：拿走所有权（i32 是 Copy，效果类似，但概念不同）
    let sum_owned: i32 = v.clone().into_iter().sum();
    println!("into_iter() 所有权求和: {}", sum_owned);

    // iter_mut()：可变借用，能改元素
    let mut v2 = vec![1, 2, 3];
    for x in v2.iter_mut() {
        *x *= 10;
    }
    println!("iter_mut() 修改后: {:?}", v2);

    // ---------- 2. 核心组合子 ----------
    // map：变换每个元素
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("map 翻倍: {:?}", doubled);

    // filter：按条件筛选
    let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("filter 偶数: {:?}", evens);

    // fold：累积（reduce）
    let total = v.iter().fold(0, |acc, x| acc + x);
    println!("fold 求和: {}", total);
    let factorial = (1..=5).fold(1, |acc, x| acc * x);
    println!("fold 阶乘 5!: {}", factorial);

    // ---------- 3. 链式组合：声明式流水线 ----------
    // 需求：1~20 中，偶数的平方，取前 3 个，求和
    let result: i32 = (1..=20)
        .filter(|x| x % 2 == 0)     // 偶数
        .map(|x| x * x)             // 平方
        .take(3)                    // 前 3 个
        .sum();                     // 求和
    // 2² + 4² + 6² = 4 + 16 + 36 = 56
    println!("流水线结果: {}", result);

    // ---------- 4. 更多组合子 ----------
    let nums = vec![5, 1, 4, 2, 3];

    // 排序（注意：排序不是惰性组合子）
    let mut sorted = nums.clone();
    sorted.sort();
    println!("排序: {:?}", sorted);

    // min / max
    println!("min: {:?}, max: {:?}", nums.iter().min(), nums.iter().max());

    // any / all：是否存在 / 是否全部
    // 注意：iter() 产生引用，闭包收到 &i32；用 copied() 得到值再比较
    println!("有偶数? {}", nums.iter().any(|x| x % 2 == 0)); // % 运算符自动解引用
    println!("全小于10? {}", nums.iter().copied().all(|x| x < 10));

    // find：找第一个满足条件的
    println!("第一个偶数: {:?}", nums.iter().find(|x| *x % 2 == 0));

    // position：找索引
    println!("数字 4 的索引: {:?}", nums.iter().position(|x| *x == 4));

    // count / enumerate
    println!("元素个数: {}", nums.iter().count());
    let indexed: Vec<(usize, &i32)> = nums.iter().enumerate().collect();
    println!("enumerate: {:?}", indexed);

    // zip：两个迭代器配对
    // names 是 [&str; 3]，iter() 产生 &&str，所以用 copied() 解引用
    let names = ["张三", "李四", "王五"];
    let scores = [88, 95, 73];
    let pairs: Vec<(&str, i32)> = names.iter().copied().zip(scores.iter().copied()).collect();
    println!("zip 配对: {:?}", pairs);

    // chain：串联两个迭代器
    let chained: Vec<i32> = nums.iter().chain(vec![100, 200].iter()).copied().collect();
    println!("chain 串联: {:?}", chained);

    // ---------- 5. 消费迭代器的方式 ----------
    // collect：收集成集合（最常用）
    // sum / product / count
    // for 循环（底层也是迭代器）
    // reduce：fold 的简化版
    let product: i32 = (1..=5).product();
    println!("product 1..=5: {}", product);

    // ---------- 6. 字符串处理的迭代器应用 ----------
    let text = "hello world rust";
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("分词: {:?}", words);

    // 大写首字母
    let capitalized: String = words
        .iter()
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    println!("首字母大写: {}", capitalized);

    // 统计单词长度
    let lengths: Vec<usize> = words.iter().map(|w| w.len()).collect();
    println!("各词长度: {:?}", lengths);

    // ---------- 7. 自定义迭代器（进阶）----------
    // 实现 Iterator trait 就能自定义迭代器
    let fib = Fibonacci::new();
    let first_10: Vec<u32> = fib.take(10).collect();
    println!("斐波那契前 10 项: {:?}", first_10);

    // 无限迭代器 + take 是经典组合
    let squares: Vec<u32> = (1u32..).map(|x| x * x).take(5).collect();
    println!("无限序列 take 5: {:?}", squares);

    // ---------- 8. 性能：零成本抽象 ----------
    // 迭代器组合子在编译期会被优化成手写循环（llvm 优化）
    // 无需担心链式调用有运行时开销
    println!("迭代器链在编译期优化为手写循环，零运行时开销 ✅");
}

// ---------- 自定义迭代器 ----------
struct Fibonacci {
    a: u32,
    b: u32,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { a: 0, b: 1 }
    }
}

// 实现 Iterator trait：只需提供 next()
impl Iterator for Fibonacci {
    type Item = u32; // 关联类型：迭代产生的元素类型

    fn next(&mut self) -> Option<u32> {
        let current = self.a;
        // ⚠️ 陷阱：必须先算好下一个数，再更新状态。
        // 错误写法：self.a = self.b; self.b = self.a + self.b;
        // 那样 self.a 已被覆盖，加出来的是 2b（得到 2 的幂，不是斐波那契）
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(current) // 永远有下一个 → 无限迭代器
    }
}
