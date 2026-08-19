// ============================================
// 第 8 课：结构体 struct —— 定义自己的数据类型
// ============================================

// 结构体 = 把相关数据打包成一个自定义类型
// 三种形式：具名字段（最常用）、元组结构体、单元结构体

// ---------- 1. 具名字段结构体 ----------
#[derive(Debug)] // 让结构体能 {:?} 打印（后面 trait 课会讲 derive）
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// ---------- 2. 元组结构体：字段没有名字，用位置访问 ----------
struct Color(i32, i32, i32);
struct Point(i32, i32);

// ---------- 3. 单元结构体：没有字段（一般配 trait 用，后续课讲）----------
struct UnitLike;

fn main() {
    // ---------- 创建结构体实例 ----------
    let user1 = User {
        email: String::from("zhangsan@example.com"),
        username: String::from("zhangsan"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1 的名字: {}", user1.username);

    // 字段可变：整个实例声明 mut 才能改字段（没有"部分字段可变"）
    let mut user2 = User {
        email: String::from("lisi@example.com"),
        username: String::from("lisi"),
        active: false,
        sign_in_count: 0,
    };
    user2.active = true; // ✅ 实例是 mut 的
    println!("user2 活跃: {}", user2.active);

    // ---------- 用函数创建（字段名就是参数名时，可简写）----------
    let user3 = build_user(String::from("wangwu"), String::from("wangwu@example.com"));
    println!("user3: {:?}", user3);

    // ---------- 结构体更新语法 ----------
    // 从 user3 复制部分字段（剩下的字段从 user1 拿）
    // 注意：email/username 是 String（非 Copy），这里会发生移动，user1 部分失效！
    let user4 = User {
        email: String::from("new@example.com"), // 新值
        ..user1 // 其余字段从 user1 复制/移动
    };
    println!("user4: {:?}", user4);
    // println!("user1: {:?}", user1); // ❌ user1 的 username/email 被移动了

    // 如果字段全是 Copy 类型，旧实例完全不受影响
    let p1 = Point(1, 2);
    let p2 = Point(5, 6);
    let distance = (p2.0 - p1.0).abs() + (p2.1 - p1.1).abs();
    println!("p1 到 p2 的曼哈顿距离: {}", distance);

    // ---------- 元组结构体 ----------
    let black = Color(0, 0, 0);
    println!("黑色 = ({}, {}, {})", black.0, black.1, black.2);
    let origin = Point(0, 0);
    println!("origin = ({}, {})", origin.0, origin.1);

    // 即使字段类型相同，Color 和 Point 是不同类型，不能互用
    // let wrong: Point = black; // ❌ 类型不匹配

    // ---------- 单元结构体 ----------
    let _unit = UnitLike; // 创建单元结构体实例（用途：配合 trait 使用，后续课展开）

    // ---------- 方法（方法 = 定义在 impl 块里的函数）----------
    let mut rect = Rectangle { width: 30, height: 50 };
    println!("面积: {}", rect.area());
    println!("正方形? {}", rect.is_square());
    println!("能否容纳另一个矩形: {}", rect.can_hold(&Rectangle { width: 20, height: 40 }));
    rect.set_width(60); // &mut self 方法：修改宽度
    println!("改宽后面积: {}", rect.area());

    // 读写字段
    println!("user3 邮箱: {}, 登录次数: {}", user3.email, user3.sign_in_count);

    // ---------- 关联函数（关联在类型上，不接收 self）----------
    // 用 :: 调用，最常见的用途是"构造函数"，如 String::from
    let square = Rectangle::square(10);
    println!("正方形: {:?}, 面积 {}", square, square.area());
}

// 字段名和参数名相同 → 简写
fn build_user(username: String, email: String) -> User {
    User {
        username, // 等价于 username: username
        email,    // 等价于 email: email
        active: true,
        sign_in_count: 1,
    }
}

// ---------- 方法的完整示例 ----------
// #[derive(Debug)] 让结构体支持 {:?} 打印
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self：只读方法（借用）
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 判断是否是正方形
    fn is_square(&self) -> bool {
        self.width == self.height
    }

    // &self 借用另一个矩形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // 关联函数（没有 self！）：用 Self:: 创建，如 Rectangle::square(10)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // 想修改 self？用 &mut self
    fn set_width(&mut self, w: u32) {
        self.width = w;
    }
}
