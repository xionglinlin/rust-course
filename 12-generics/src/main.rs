// ============================================
// 第 12 课：泛型 Generics —— 一次编写，多种类型
// ============================================

// 泛型：把"类型"本身当作参数。写一份代码，适用于任意类型。
// 你已经见过很多泛型：Vec<T>、Option<T>、Result<T, E>、HashMap<K, V>

fn main() {
    // ---------- 1. 泛型函数 ----------
    // 没有泛型时，每种类型写一遍：
    fn largest_i32(list: &[i32]) -> i32 {
        let mut max = list[0];
        for &item in list {
            if item > max {
                max = item;
            }
        }
        max
    }

    let nums = vec![34, 50, 25, 100, 65];
    println!("最大 i32: {}", largest_i32(&nums));

    // 泛型版本：<T: PartialOrd> 表示 T 必须支持比较（trait 约束，第 13 课细讲）
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut max = list[0];
        for &item in list {
            if item > max {
                max = item;
            }
        }
        max
    }

    println!("最大整数: {}", largest(&vec![1, 5, 3, 9]));
    println!("最大浮点: {}", largest(&vec![1.5, 2.7, 0.3]));
    println!("最大字符: {}", largest(&vec!['a', 'z', 'm']));
    // 同一个函数，三种类型都能用！

    // ---------- 2. 泛型结构体 ----------
    // 经典的 Point<T>：x 和 y 是同一类型 T
    let int_point = Point { x: 5, y: 10 };          // Point<i32>
    let float_point = Point { x: 1.5, y: 2.5 };     // Point<f64>
    println!("整数点: {:?}", int_point);
    println!("浮点点: {:?}", float_point);

    // 两种不同类型的点：
    let mixed = Point2 { x: 5, y: 2.5 };            // Point2<i32, f64>
    println!("混合点: {:?}", mixed);

    // ---------- 3. 泛型枚举 ----------
    // Option<T>、Result<T, E> 就是泛型枚举，我们一直在用！
    let some_num: Option<i32> = Some(5);
    let some_str: Option<&str> = Some("你好");
    println!("{:?} {:?}", some_num, some_str);

    // 自己定义一个泛型枚举
    let r1: MyResult<i32, String> = MyResult::Ok(42);
    let r2: MyResult<i32, String> = MyResult::Err(String::from("出错了"));
    println!("{:?} {:?}", r1, r2);

    // ---------- 4. 泛型方法（impl 块）----------
    println!("x 坐标: {}", int_point.x());
    println!("距离原点: {}", float_point.distance_from_origin());

    // impl<T> 里的方法适用于所有 T（Point<T> 才有 x()；Point2 直接访问字段）
    println!("混合点的 x: {}, y: {}", mixed.x, mixed.y);

    // 针对特定类型的方法：只对 Point<f64> 有效
    // float_point.distance_from_origin() 在上面调用了 ✅
    // int_point.distance_from_origin() 不存在（i32 不能开方），
    // 取消注释下面这行会报错：no method named `distance_from_origin`
    // println!("{}", int_point.distance_from_origin());

    // ---------- 5. 泛型的性能：零成本抽象 ----------
    // 泛型在编译期做"单态化"（monomorphization）：编译器为每种具体类型
    // 生成一份专用代码。所以运行时的性能和手写每种类型一模一样，
    // 没有虚函数表、没有装箱、没有运行时开销——这就是"零成本抽象"。
    println!("编译期单态化，运行时无开销 ✅");

    // ---------- 6. 泛型实战：自己的 Vec 包装 ----------
    let mut bag = Bag::new();
    bag.put(String::from("苹果"));
    bag.put(String::from("香蕉"));
    println!("袋子里: {:?}", bag);
    println!("拿出来: {:?}", bag.take());
    println!("剩: {:?}", bag);
}

// ---------- 泛型结构体：两个字段同类型 ----------
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// ---------- 泛型结构体：两个字段不同类型 ----------
#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

// ---------- 泛型枚举 ----------
#[derive(Debug)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

// ---------- 泛型方法 ----------
impl<T> Point<T> {
    // 所有 Point<T> 都有 x() 方法
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    // 只有 Point<f64> 才有这个方法（f64 才有 sqrt）
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// ---------- 泛型实战：简易 Bag ----------
#[derive(Debug)]
struct Bag<T> {
    items: Vec<T>,
}

impl<T> Bag<T> {
    fn new() -> Bag<T> {
        Bag { items: Vec::new() }
    }

    fn put(&mut self, item: T) {
        self.items.push(item);
    }

    fn take(&mut self) -> Option<T> {
        self.items.pop()
    }
}
