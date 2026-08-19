// ============================================
// 第 26 课：实战项目四 —— 迷你 JSON 解析器
// ============================================
// 手写一个 JSON 解析器：解析 → Value 枚举 → 打印。
// 这是第 23 课"递归下降"的终极形态，也是 serde_json 的核心思想。
//
// 复习：
//   - 递归枚举（Value 包含自己）     - 递归下降解析
//   - match + 错误处理               - 字符/字节处理
//   - HashMap + Vec                  - Display trait
//   - 转义序列（\n \" \uXXXX）       - 单元测试
//
// 用法：cargo run -- '{"name":"小明","age":18,"scores":[90,95,88]}'

use std::collections::HashMap;
use std::fmt;

// ---------- 1. JSON 数据模型：递归枚举 ----------
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

// ---------- 2. 解析器：基于字符（&str + char 游标）的递归下降 ----------
// 为什么不用字节？UTF-8 的中文是多字节的，按字节切会拆散字符（乱码）。
// 用 char 游标：peek/next 天然处理多字节字符。
pub struct Parser<'a> {
    input: &'a str, // 输入
    pos: usize,     // 当前位置（字节索引）
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser { input, pos: 0 }
    }

    // 看当前字符（不消费）
    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    // 消费当前字符
    fn next(&mut self) -> Option<char> {
        let c = self.input[self.pos..].chars().next()?;
        self.pos += c.len_utf8(); // 按字符的实际字节数前进
        Some(c)
    }

    // 跳过空白：空格、制表符、换行、回车
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }
    }

    // 期望某个字符，否则报错
    fn expect(&mut self, expected: char) -> Result<(), String> {
        self.skip_whitespace();
        match self.next() {
            Some(c) if c == expected => Ok(()),
            other => Err(format!(
                "位置 {}: 期望 '{}'，实际 {:?}",
                self.pos, expected, other
            )),
        }
    }

    // ---------- 入口 ----------
    pub fn parse(&mut self) -> Result<Value, String> {
        let value = self.parse_value()?;
        self.skip_whitespace();
        if self.peek().is_some() {
            return Err(format!("位置 {}: 解析结束后还有多余内容", self.pos));
        }
        Ok(value)
    }

    // ---------- 分派 ----------
    fn parse_value(&mut self) -> Result<Value, String> {
        self.skip_whitespace();
        match self.peek() {
            Some('{') => self.parse_object(),
            Some('[') => self.parse_array(),
            Some('"') => Ok(Value::String(self.parse_string()?)),
            Some('t') => self.parse_literal("true", Value::Bool(true)),
            Some('f') => self.parse_literal("false", Value::Bool(false)),
            Some('n') => self.parse_literal("null", Value::Null),
            Some(c) if c == '-' || c.is_ascii_digit() => Ok(Value::Number(self.parse_number()?)),
            other => Err(format!("位置 {}: 意外的字符 {:?}", self.pos, other)),
        }
    }

    // ---------- 对象 ----------
    fn parse_object(&mut self) -> Result<Value, String> {
        self.expect('{')?;
        let mut map = HashMap::new();

        self.skip_whitespace();
        if self.peek() == Some('}') {
            self.pos += 1;
            return Ok(Value::Object(map)); // 空对象 {}
        }

        loop {
            self.skip_whitespace();
            let key = self.parse_string()?; // 键必须是字符串
            self.expect(':')?;
            let value = self.parse_value()?;
            map.insert(key, value);

            self.skip_whitespace();
            match self.next() {
                Some(',') => continue, // 还有下一对
                Some('}') => break,    // 结束
                other => {
                    return Err(format!(
                        "位置 {}: 对象里期望 ',' 或 '}}'，实际 {:?}",
                        self.pos, other
                    ))
                }
            }
        }
        Ok(Value::Object(map))
    }

    // ---------- 数组 ----------
    fn parse_array(&mut self) -> Result<Value, String> {
        self.expect('[')?;
        let mut items = Vec::new();

        self.skip_whitespace();
        if self.peek() == Some(']') {
            self.pos += 1;
            return Ok(Value::Array(items)); // 空数组 []
        }

        loop {
            let value = self.parse_value()?;
            items.push(value);

            self.skip_whitespace();
            match self.next() {
                Some(',') => continue,
                Some(']') => break,
                other => {
                    return Err(format!(
                        "位置 {}: 数组里期望 ',' 或 ']'，实际 {:?}",
                        self.pos, other
                    ))
                }
            }
        }
        Ok(Value::Array(items))
    }

    // ---------- 字符串（含转义）----------
    fn parse_string(&mut self) -> Result<String, String> {
        self.expect('"')?;
        let mut result = String::new();

        loop {
            match self.next() {
                Some('"') => break, // 结束引号
                Some('\\') => {
                    // 转义序列
                    match self.next() {
                        Some('"') => result.push('"'),
                        Some('\\') => result.push('\\'),
                        Some('/') => result.push('/'),
                        Some('b') => result.push('\u{0008}'),
                        Some('f') => result.push('\u{000C}'),
                        Some('n') => result.push('\n'),
                        Some('r') => result.push('\r'),
                        Some('t') => result.push('\t'),
                        Some('u') => {
                            // \uXXXX：4 位十六进制
                            let code = self.parse_hex4()?;
                            result.push(char::from_u32(code).ok_or("非法 Unicode 码点")?);
                        }
                        other => return Err(format!("位置 {}: 非法转义 {:?}", self.pos, other)),
                    }
                }
                Some(c) if c >= '\u{20}' => {
                    // 普通字符（含中文等非 ASCII——char 游标保证完整性）
                    result.push(c);
                }
                // 其他情况：字符串未闭合、控制字符或输入耗尽
                _ => return Err(format!("位置 {}: 字符串未闭合或含控制字符", self.pos)),
            }
        }
        Ok(result)
    }
    // 解析 \uXXXX 的 4 位十六进制
    fn parse_hex4(&mut self) -> Result<u32, String> {
        let mut code = 0u32;
        for _ in 0..4 {
            let c = self.next().ok_or("\\u 后不足 4 位十六进制")?;
            let digit = match c {
                '0'..='9' => c as u32 - '0' as u32,
                'a'..='f' => c as u32 - 'a' as u32 + 10,
                'A'..='F' => c as u32 - 'A' as u32 + 10,
                _ => return Err(format!("非法十六进制字符: {}", c)),
            };
            code = code * 16 + digit;
        }
        Ok(code)
    }

    // ---------- 数字 ----------
    fn parse_number(&mut self) -> Result<f64, String> {
        let start = self.pos;
        // 负号
        if self.peek() == Some('-') {
            self.pos += 1;
        }
        // 整数部分
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.pos += 1;
            } else {
                break;
            }
        }
        // 小数部分
        if self.peek() == Some('.') {
            self.pos += 1;
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
        // 指数部分
        if matches!(self.peek(), Some('e') | Some('E')) {
            self.pos += 1;
            if matches!(self.peek(), Some('+') | Some('-')) {
                self.pos += 1;
            }
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }

        let text = &self.input[start..self.pos];
        text.parse::<f64>().map_err(|_| format!("非法数字: {}", text))
    }

    // ---------- 字面量 true/false/null ----------
    fn parse_literal(&mut self, word: &str, value: Value) -> Result<Value, String> {
        for expected in word.chars() {
            match self.next() {
                Some(c) if c == expected => {}
                other => {
                    return Err(format!(
                        "位置 {}: 期望 '{}'，实际 {:?}",
                        self.pos, expected, other
                    ))
                }
            }
        }
        Ok(value)
    }
}

// ---------- 3. 打印：实现 Display ----------
// 简化版输出（不保证和输入字节完全一致，仅用于演示）
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Array(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Object(map) => {
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}

// ---------- 4. 顶层接口 ----------
pub fn parse_json(input: &str) -> Result<Value, String> {
    let mut parser = Parser::new(input);
    parser.parse()
}

// ---------- 5. 访问辅助方法 ----------
impl Value {
    // 按路径取对象字段：json.get("name")
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Object(map) => map.get(key),
            _ => None,
        }
    }

    // 转为数字（Option）
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
}

// ---------- 6. main ----------
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let json = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        // 默认示例
        r#"{"name":"小明","age":18,"tags":["学生","rust爱好者"],"address":{"city":"北京","zip":"100000"},"score":88.5,"active":true,"note":null}"#.to_string()
    };

    match parse_json(&json) {
        Ok(value) => {
            println!("解析成功！\n");
            println!("{}", value);
            println!("\n--- 访问测试 ---");
            if let Some(name) = value.get("name") {
                println!("name = {:?}", name.as_str());
            }
            if let Some(age) = value.get("age") {
                println!("age = {:?}", age.as_number());
            }
        }
        Err(e) => {
            println!("解析失败: {}", e);
            std::process::exit(1);
        }
    }
}

// ---------- 7. 单元测试 ----------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_primitives() {
        assert_eq!(parse_json("null").unwrap(), Value::Null);
        assert_eq!(parse_json("true").unwrap(), Value::Bool(true));
        assert_eq!(parse_json("false").unwrap(), Value::Bool(false));
        assert_eq!(parse_json("42").unwrap(), Value::Number(42.0));
        assert_eq!(parse_json("-3.14").unwrap(), Value::Number(-3.14));
        assert_eq!(parse_json("1e3").unwrap(), Value::Number(1000.0));
    }

    #[test]
    fn test_parse_string_and_escapes() {
        assert_eq!(parse_json("\"hello\"").unwrap(), Value::String("hello".into()));
        // 转义序列：JSON 内容为 "a\nb\tc\""（含 \n \t \" 三种转义）
        // 用普通字符串字面量写：每个 \ 写成 \\，每个 " 写成 \"
        let v = parse_json("\"a\\nb\\tc\\\"\"").unwrap();
        assert_eq!(v, Value::String("a\nb\tc\"".into()));
        // Unicode
        let v = parse_json(r#""\u4f60\u597d""#).unwrap();
        assert_eq!(v, Value::String("你好".into()));
    }

    #[test]
    fn test_parse_array() {
        let v = parse_json("[1, 2, 3]").unwrap();
        assert_eq!(
            v,
            Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ])
        );
        // 嵌套
        let v = parse_json("[[1, 2], [3, 4]]").unwrap();
        assert!(matches!(v, Value::Array(_)));
    }

    #[test]
    fn test_parse_object() {
        let v = parse_json(r#"{"a": 1, "b": "x"}"#).unwrap();
        assert_eq!(v.get("a").and_then(|x| x.as_number()), Some(1.0));
        assert_eq!(v.get("b").and_then(|x| x.as_str()), Some("x"));
    }

    #[test]
    fn test_complex() {
        let json = r#"
        {
            "name": "小明",
            "age": 18,
            "scores": [90, 95, 88],
            "info": { "city": "北京", "hobby": ["rust", "music"] },
            "ok": true,
            "nothing": null
        }"#;
        let v = parse_json(json).unwrap();
        assert_eq!(v.get("age").and_then(|x| x.as_number()), Some(18.0));
        assert_eq!(v.get("nothing").unwrap(), &Value::Null);
    }

    #[test]
    fn test_errors() {
        assert!(parse_json("").is_err()); // 空输入
        assert!(parse_json("{").is_err()); // 未闭合
        assert!(parse_json("[1, 2").is_err()); // 数组未闭合
        assert!(parse_json(r#"{"a": }"#).is_err()); // 值缺失
        assert!(parse_json("tru").is_err()); // 截断字面量
        assert!(parse_json(r#""abc"#).is_err()); // 字符串未闭合
        assert!(parse_json("1 2").is_err()); // 多余内容
    }

    #[test]
    fn test_whitespace_tolerance() {
        assert_eq!(parse_json("  {  \"a\" : 1 }  ").unwrap().get("a").and_then(|x| x.as_number()), Some(1.0));
    }
}
