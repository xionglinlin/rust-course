// ============================================
// 第 14 课：生命周期（Lifetimes）
// ============================================

// 生命周期（lifetime）：一个引用"有效"的时间范围。
// 之前讲过：借用必须比借出者"活得更短"。
// 编译器大部分时候能自动推断生命周期（省略规则），
// 但函数返回引用、结构体存引用时，需要显式标注。

// 标注语法：'a 是生命周期参数，类似泛型 T
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
// 意思是：返回的引用，有效期 = x 和 y 中较短的那个

fn main() {
    // ---------- 1. 为什么需要生命周期标注 ----------
    // 下面的函数想返回两个字符串中较长的一个：
    // 问题：返回的引用到底借用谁？编译器不知道返回的引用能活多久
    // fn longest_broken(x: &str, y: &str) -> &str {  // ❌ 缺生命周期标注
    //     if x.len() > y.len() { x } else { y }
    // }

    // 加上 'a 标注后就明白了
    let s1 = String::from("短");
    let s2 = String::from("比较长的字符串");
    let result = longest(s1.as_str(), s2.as_str());
    println!("较长的是: {}", result);

    // ---------- 2. 生命周期注解保护了什么 ----------
    // 防止"悬垂引用"：返回的引用不能比它借用的数据活得更久
    {
        let short_lived = String::from("短命");
        let r = longest(s1.as_str(), short_lived.as_str()); // ✅ 同作用域内用没问题
        println!("r 在作用域内用: {}", r);
    } // r 和 short_lived 一起在这里被释放
    // 如果把 r 声明在外层再用，编译器会报 "does not live long enough"：
    //     let r2: &str;
    //     { let sl = String::from("x"); r2 = longest(s1.as_str(), sl.as_str()); }
    //     println!("{}", r2); // ❌ sl 已释放，r2 悬垂——编译期拦截！

    // ---------- 3. 生命周期省略规则（不需要标注的情况）----------
    // 规则1：每个引用参数都有自己的生命周期
    // 规则2：只有一个引用参数时，返回的生命周期 = 该参数
    // 规则3：多个参数，如果其中有 &self，返回的生命周期 = &self
    // 所以下面这些函数不用写标注：
    fn first_word(s: &str) -> &str { s }  // 规则2：返回借用 s
    let fw = first_word("hello world");
    println!("first_word 省略标注也能用: {}", fw);

    // ---------- 4. 结构体中的生命周期 ----------
    // 结构体想存引用，必须标注生命周期
    struct Excerpt<'a> {
        part: &'a str, // 这个引用至少活得和结构体一样久
    }

    let novel = String::from("从前有座山，山里有座庙。庙里有个老和尚。");
    let first_sentence = novel.split('。').next().expect("没有找到句子");
    let excerpt = Excerpt { part: first_sentence }; // Excerpt<'_>
    println!("摘录: {}", excerpt.part);
    // excerpt 借用 novel，只要 novel 还活着，excerpt 就有效

    // ---------- 5. 'static 生命周期 ----------
    // 'static：引用存活于整个程序运行期间
    // 字符串字面量就是 &'static str
    let greeting: &'static str = "我活到程序结束";
    println!("{}", greeting);

    // ---------- 6. 实战：更真实的例子 ----------
    // 两个字符串，取较长者的完整版本（含所有权的思考）
    let a = String::from("rust");
    let b = String::from("programming");
    let longer = longest(a.as_str(), b.as_str());
    println!("较长者: {}", longer);

    // 生命周期 + 泛型 + trait 的组合
    let x = 5;
    let y = 10;
    let bigger = max_ref(&x, &y);
    println!("较大的引用指向: {}", bigger);

    // ---------- 7. 感受编译器：生命周期错误 ----------
    // 我们验证过：dangle 函数返回局部变量引用 → E0106
    // 生命周期标注的价值：把"引用有效性"的证明交给编译器，
    // 悬垂引用、use-after-free 在 C/C++ 里是运行时灾难，Rust 编译期拦截
    println!("编译器保证了引用永远有效 ✅");
}

// 生命周期标注：返回的引用生命周期 = 两个参数中较短的那个
// 'a 是泛型生命周期参数，编译器会检查约束
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 生命周期 + 泛型 + trait bound 组合示例
// T: Copy 因为要返回引用，需要 deref 复制
fn max_ref<'a, T>(x: &'a T, y: &'a T) -> &'a T
where
    T: PartialOrd,
{
    if x > y { x } else { y }
}
