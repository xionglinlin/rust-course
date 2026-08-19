// ============================================
// 第 11 课：错误处理 —— panic、Result、? 运算符
// ============================================

// Rust 的两种错误：
// 1. 不可恢复错误：panic!（程序崩溃退出）
// 2. 可恢复错误：Result<T, E>（调用方决定怎么处理）
//
// Result 是标准库枚举：
// enum Result<T, E> { Ok(T), Err(E) }

use std::fs::File;
use std::io::{self, Read};

fn main() {
    // ---------- 1. panic!：不可恢复错误 ----------
    // 显式 panic
    // panic!("程序出错了！"); // 取消注释：程序立即崩溃

    // 隐式 panic：越界、unwrap 遇 None 等
    // let v = vec![1];
    // v[99]; // panicked: index out of bounds

    // 生产代码里 panic 的用途：程序状态被破坏、无法继续执行时
    // 比如配置缺失、数据库连接失败等"不该发生"的事

    // ---------- 2. Result：可恢复错误 ----------
    // 经典场景：打开文件
    let result = File::open("不存在的文件.txt");
    println!("result 的类型: {}", std::any::type_name_of_val(&result));

    match result {
        Ok(file) => println!("打开成功: {:?}", file),
        Err(e) => println!("打开失败: {}", e), // 错误可以打印！
    }

    // ---------- 3. 自己返回 Result ----------
    let r1 = divide_result(10, 2);
    let r2 = divide_result(10, 0);
    println!("10/2 = {:?}", r1);
    println!("10/0 = {:?}", r2);

    // 用 match 处理
    match r1 {
        Ok(v) => println!("结果是 {}", v),
        Err(msg) => println!("出错了: {}", msg),
    }

    // ---------- 4. unwrap / expect：快速取值（有 panic 风险）----------
    // 适合：测试、原型、确定不会出错的地方
    let ok = divide_result(10, 2).unwrap();        // 成功就取值
    println!("unwrap 得到: {}", ok);
    // let bad = divide_result(10, 0).unwrap();    // ❌ None/Err 时 panic！
    // expect 可以给 panic 信息
    let val = "42".parse::<i32>().expect("解析失败");
    println!("expect 得到: {}", val);

    // ---------- 5. unwrap_or / unwrap_or_else：优雅兜底 ----------
    let safe = divide_result(10, 0).unwrap_or(-1);       // Err 时用默认值
    println!("兜底: {}", safe);
    let safe2 = divide_result(10, 0).unwrap_or_else(|e| {
        println!("错误信息: {}", e);
        -1 // 闭包计算兜底值（第 17 课讲闭包）
    });
    println!("闭包兜底: {}", safe2);

    // ---------- 6. ? 运算符：错误的自动传播（最常用！）----------
    // ? 等价于：match { Ok(v) => v, Err(e) => return Err(e.into()) }
    // 函数返回 Result 时，用 ? 一路向上抛错误，代码极其简洁
    // 注意：cargo run 的工作目录是"当前终端目录"而非项目目录，
    // 所以用 CARGO_MANIFEST_DIR（编译期常量，指向项目根）拼绝对路径最稳妥
    let project_dir = env!("CARGO_MANIFEST_DIR");

    // 成功路径：hello.txt 存在
    match read_file_content(&format!("{}/hello.txt", project_dir)) {
        Ok(name) => println!("读取到用户名: {}", name),
        Err(e) => println!("读取失败: {}", e),
    }
    // 失败路径：文件不存在时 ? 自动 return Err
    match read_file_content(&format!("{}/不存在.txt", project_dir)) {
        Ok(c) => println!("文件内容: {}", c),
        Err(e) => println!("? 传播了错误: {}", e),
    }

    // ---------- 7. 组合子：map / and_then ----------
    // map：Ok 时转换值
    let r = divide_result(10, 2).map(|v| v * 100);
    println!("map 后: {:?}", r); // Ok(500)

    // 链式处理
    let result = divide_result(100, 4)
        .map(|v| v + 1)
        .map(|v| v * 2)
        .unwrap_or(-1);
    println!("链式: {}", result);

    // ---------- 8. 自定义错误类型（进阶预告）----------
    // 大型项目里会定义自己的错误枚举，实现 Display/Error trait
    // 第 14 课讲 trait 后回来完善这个例子
    match check_age(-5) {
        Ok(()) => println!("年龄合法"),
        Err(AgeError::Negative) => println!("年龄不能为负数"),
        Err(AgeError::TooOld) => println!("年龄太大了"),
    }
}

// 返回 Result：除零返回 Err
// 注意错误类型 String——简单但丢失类型信息，进阶课会优化
fn divide_result(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

// ? 运算符的完整示例：读取文件内容
// 函数签名里的 io::Result<String> = Result<String, io::Error>
// 泛型参数化：任意路径都行
fn read_file_content(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?; // 失败就 return Err，成功拿到 File
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // 失败同样传播
    Ok(contents)
}

// 自定义错误枚举
enum AgeError {
    Negative,
    TooOld,
}

fn check_age(age: i32) -> Result<(), AgeError> {
    if age < 0 {
        Err(AgeError::Negative)
    } else if age > 150 {
        Err(AgeError::TooOld)
    } else {
        Ok(())
    }
}
