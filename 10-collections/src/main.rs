// ============================================
// 第 10 课：集合 —— Vec、HashMap、HashSet
// ============================================

use std::collections::{HashMap, HashSet};

fn main() {
    // ============ 一、Vec<T>：动态数组 ============
    // 可变长度、同类型元素，存在堆上

    // 创建
    let mut v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3]; // 宏创建
    println!("v2 = {:?}", v2);

    // 添加/删除
    v1.push(10);
    v1.push(20);
    v1.push(30);
    println!("v1 = {:?}", v1);
    v1.pop(); // 弹出末尾
    println!("pop 后 v1 = {:?}", v1);

    // 读取：索引 [] 越界会 panic；.get() 返回 Option，安全
    println!("v2[0] = {}", v2[0]);
    println!("v2.get(1) = {:?}", v2.get(1));   // Some(&2)
    println!("v2.get(99) = {:?}", v2.get(99)); // None，不会 panic

    // 遍历
    for x in &v2 { // &v2 只读借用
        print!("{} ", x);
    }
    println!();

    // 修改遍历：&mut
    for x in &mut v2 {
        *x *= 2; // 每个元素翻倍
    }
    println!("翻倍后 v2 = {:?}", v2);

    // 所有权注意：push 进 Vec 后，元素的所有权归 Vec 所有
    let s1 = String::from("hi");
    let mut strs = vec![s1]; // s1 被移动进 Vec
    // println!("{}", s1); // ❌ 不能再用
    strs.push(String::from("rust"));
    println!("strs = {:?}", strs);

    // 常用方法
    println!("长度: {}", v2.len());
    println!("是否为空: {}", v2.is_empty());
    println!("第一个: {:?}, 最后一个: {:?}", v2.first(), v2.last());
    v2.sort();
    println!("排序: {:?}", v2);
    v2.reverse();
    println!("反转: {:?}", v2);
    println!("包含 4? {}", v2.contains(&4));

    // 从迭代器收集（第 17 课深入）
    let squares: Vec<i32> = (1..=5).map(|x| x * x).collect();
    println!("平方数: {:?}", squares);

    // ============ 二、HashMap<K, V>：键值对 ============
    let mut scores = HashMap::new();
    scores.insert(String::from("数学"), 95);
    scores.insert(String::from("语文"), 88);
    scores.insert(String::from("英语"), 91);
    println!("成绩表: {:?}", scores);

    // 读取：.get() 返回 Option<&V>
    match scores.get("数学") {
        Some(score) => println!("数学成绩: {}", score),
        None => println!("没有数学成绩"),
    }
    // 不存在的键
    println!("物理成绩: {:?}", scores.get("物理"));

    // 遍历（顺序不保证）
    for (subject, score) in &scores {
        println!("{}: {}", subject, score);
    }

    // 更新：
    // entry API：键不存在才插入（"语文"已存在，不会覆盖 88）
    scores.entry(String::from("语文")).or_insert(60);
    // 键不存在时插入默认值（"物理"不存在 → 插入 60）
    scores.entry(String::from("物理")).or_insert(60);
    println!("更新后: {:?}", scores);

    // 覆盖插入
    scores.insert(String::from("数学"), 100);
    println!("数学改 100 后: {:?}", scores.get("数学"));

    // 实战：统计单词出现次数（entry 的经典用法）
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1; // or_insert 返回 &mut V，直接改
    }
    println!("单词统计: {:?}", word_count);
    println!("'the' 出现 {} 次", word_count["the"]);

    // ============ 三、HashSet<T>：无序不重复集合 ============
    let mut set = HashSet::new();
    set.insert("苹果");
    set.insert("香蕉");
    set.insert("苹果"); // 重复插入无效
    println!("集合: {:?}, 大小: {}", set, set.len());
    println!("含苹果? {}", set.contains("苹果"));
    set.remove("香蕉");
    println!("移除后: {:?}", set);

    // 集合运算：交集、并集、差集
    let a: HashSet<i32> = [1, 2, 3, 4].iter().copied().collect();
    let b: HashSet<i32> = [3, 4, 5, 6].iter().copied().collect();
    println!("a = {:?}, b = {:?}", a, b);
    println!("交集: {:?}", a.intersection(&b).collect::<Vec<_>>());
    println!("并集: {:?}", a.union(&b).collect::<Vec<_>>());
    println!("a 差 b: {:?}", a.difference(&b).collect::<Vec<_>>());

    // 去重实战
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    let unique: HashSet<i32> = nums.iter().copied().collect();
    println!("去重后: {:?}", unique);

    // ============ 四、String 也是集合 ============
    // String 本质是 Vec<u8>，前面课已讲，这里做个串联
    let mut s = String::from("Rust");
    s.push_str(" 很棒");
    println!("String 集合用法: {}", s);

    // 小结：
    // Vec: 有序、可重复、随机访问 —— "列表"
    // HashMap: 键值对、O(1) 查找 —— "字典"
    // HashSet: 无序、去重、O(1) 成员判断 —— "集合"
}
