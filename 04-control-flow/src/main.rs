// ============================================
// 第 4 课：控制流 —— if/else、loop、while、for
// ============================================

fn main() {
    // ---------- 1. if / else if / else ----------
    // 注意：条件不需要加括号，且必须是 bool 类型（不会自动转换！）
    let score = 85;

    if score >= 90 {
        println!("优秀");
    } else if score >= 80 {
        println!("良好");
    } else if score >= 60 {
        println!("及格");
    } else {
        println!("不及格");
    }

    // if 是表达式！可以赋值给变量（每个分支必须是同一类型）
    let is_even = if score % 2 == 0 { "偶数" } else { "奇数" };
    println!("score 是{}", is_even);

    // ---------- 2. loop：无限循环 ----------
    // 用 break 退出；break 还可以带出值（loop 也是表达式！）
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break 带值，loop 表达式的结果
        }
    };
    println!("loop 返回了 {}", result);

    // ---------- 3. while：条件循环 ----------
    let mut n = 3;
    while n > 0 {
        println!("倒计时 {}", n);
        n -= 1;
    }

    // ---------- 4. for：遍历集合 ----------
    // 最常用的循环，遍历数组
    let arr = [10, 20, 30, 40];
    for item in arr {
        println!("arr 中的元素: {}", item);
    }

    // 遍历范围 1..=5（含 5）；1..5 则不含 5
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!(); // 换行

    // 按索引遍历：用 .iter().enumerate() 拿到 (索引, 元素)
    for (idx, value) in arr.iter().enumerate() {
        println!("arr[{}] = {}", idx, value);
    }

    // ---------- 5. 循环标签：break/continue 指定跳出哪一层 ----------
    // 用于跳出嵌套循环
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            if i == 2 && j == 2 {
                continue 'outer; // 跳到外层循环的下一次迭代
            }
            println!("i={}, j={}", i, j);
        }
    }

    // ---------- 6. continue：跳过本次迭代 ----------
    // 打印 1~10 中的偶数
    for i in 1..=10 {
        if i % 2 != 0 {
            continue;
        }
        print!("{} ", i);
    }
    println!();
}
