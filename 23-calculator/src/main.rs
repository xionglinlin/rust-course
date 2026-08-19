// ============================================
// 第 23 课：实战项目 —— 命令行计算器
// ============================================
// 综合运用前 22 课知识：
//   - 枚举（Token、Operator）       - 模式匹配（match）
//   - 错误处理（Result + ?）        - 字符串/迭代器
//   - 结构体 + impl                 - 泛型方法
//   - 单元测试（#[cfg(test)]）      - 模块化思维
//   - 表达式求值：递归下降解析器（经典算法）
//
// 用法：
//   cargo run -- "1 + 2 * 3"    # 命令行传表达式
//   cargo run                   # 进入交互模式（输入 quit 退出）
//   cargo test                  # 运行单元测试

use std::io::{self, Write};

// ---------- 1. Token：词法单元 ----------
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,     // ^ 幂运算
    LParen,    // (
    RParen,    // )
}

// ---------- 2. 运算符优先级 ----------
// 返回元组：(优先级, 结合性)。true = 左结合，false = 右结合
fn precedence(op: &Token) -> Option<(u8, bool)> {
    match op {
        Token::Plus | Token::Minus => Some((1, true)),
        Token::Star | Token::Slash => Some((2, true)),
        Token::Caret => Some((3, false)), // 幂运算右结合：2^3^2 = 2^(3^2)
        _ => None,
    }
}

// ---------- 3. 词法分析：字符串 → Token 列表 ----------
fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable(); // peekable：可以偷看下一个字符

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' => {
                chars.next(); // 跳过空白
            }
            '0'..='9' | '.' => {
                // 收集数字（支持小数）：123.45
                let mut num_str = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() || d == '.' {
                        num_str.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match num_str.parse::<f64>() {
                    Ok(n) => tokens.push(Token::Number(n)),
                    Err(_) => return Err(format!("无效数字: {}", num_str)),
                }
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Star);
            }
            '/' => {
                chars.next();
                tokens.push(Token::Slash);
            }
            '^' => {
                chars.next();
                tokens.push(Token::Caret);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            other => return Err(format!("无法识别的字符: '{}'", other)),
        }
    }
    Ok(tokens)
}

// ---------- 4. 解析器：Token 列表 → 求值 ----------
// 递归下降 + 优先级爬升（经典算法，几十行实现完整优先级）
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, pos: 0 }
    }

    // 看当前 token（不消费）
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    // 消费当前 token
    fn next(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        t
    }

    // 入口：解析整个表达式
    fn parse(&mut self) -> Result<f64, String> {
        let value = self.parse_expr(0)?;
        // 表达式结束后必须是 EOF（不能有残留 token）
        if self.peek().is_some() {
            return Err(format!("意外的 token: {:?}", self.peek().unwrap()));
        }
        Ok(value)
    }

    // 优先级爬升：min_prec 是允许的最低优先级
    fn parse_expr(&mut self, min_prec: u8) -> Result<f64, String> {
        // 先解析一个"原子"（数字或括号表达式）
        let mut lhs = self.parse_atom()?;

        // 循环处理运算符
        loop {
            let op = match self.peek() {
                Some(t) if precedence(t).is_some() => t.clone(),
                _ => break, // 不是运算符（或到末尾）就结束
            };
            let (prec, left_assoc) = precedence(&op).unwrap();

            if prec < min_prec {
                break; // 优先级不够，交给上层处理
            }

            self.next(); // 消费运算符

            // 右结合时，右侧用 prec+1 作为最低优先级（左边用 prec）
            let next_min = if left_assoc { prec + 1 } else { prec };
            let rhs = self.parse_expr(next_min)?;
            lhs = apply_op(&op, lhs, rhs)?;
        }

        Ok(lhs)
    }

    // 解析原子：数字 或 ( 表达式 )
    fn parse_atom(&mut self) -> Result<f64, String> {
        match self.next() {
            Some(Token::Number(n)) => Ok(n),
            Some(Token::LParen) => {
                let value = self.parse_expr(0)?; // 括号内是完整表达式
                match self.next() {
                    Some(Token::RParen) => Ok(value),
                    _ => Err(String::from("缺少右括号 )")),
                }
            }
            Some(other) => Err(format!("意外的 token: {:?}", other)),
            None => Err(String::from("表达式不完整")),
        }
    }
}

// 执行二元运算（除零检查）
fn apply_op(op: &Token, a: f64, b: f64) -> Result<f64, String> {
    match op {
        Token::Plus => Ok(a + b),
        Token::Minus => Ok(a - b),
        Token::Star => Ok(a * b),
        Token::Slash => {
            if b == 0.0 {
                Err(String::from("除数不能为零"))
            } else {
                Ok(a / b)
            }
        }
        Token::Caret => Ok(a.powf(b)),
        _ => Err(format!("不是二元运算符: {:?}", op)),
    }
}

// ---------- 5. 顶层接口 ----------
fn evaluate(expr: &str) -> Result<f64, String> {
    let tokens = tokenize(expr)?;
    if tokens.is_empty() {
        return Err(String::from("空表达式"));
    }
    let mut parser = Parser::new(tokens);
    parser.parse()
}

// ---------- 6. 交互模式（REPL）----------
fn repl() {
    println!("计算器 REPL（输入 quit 退出）");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            break;
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "quit" || line == "exit" {
            break;
        }
        match evaluate(line) {
            Ok(result) => println!("= {}", result),
            Err(e) => println!("错误: {}", e),
        }
    }
}

// ---------- 7. main ----------
fn main() {
    // 命令行参数模式：cargo run -- "1 + 2 * 3"
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let expr = args[1..].join(" ");
        match evaluate(&expr) {
            Ok(result) => println!("{} = {}", expr, result),
            Err(e) => println!("错误: {}", e),
        }
    } else {
        // 交互模式
        repl();
    }
}

// ---------- 8. 单元测试 ----------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_add() {
        assert_eq!(evaluate("1 + 2").unwrap(), 3.0);
    }

    #[test]
    fn test_precedence() {
        // 乘法优先于加法
        assert_eq!(evaluate("1 + 2 * 3").unwrap(), 7.0);
        assert_eq!(evaluate("(1 + 2) * 3").unwrap(), 9.0);
    }

    #[test]
    fn test_division_and_negative() {
        assert_eq!(evaluate("10 / 4").unwrap(), 2.5);
        assert_eq!(evaluate("5 - 8").unwrap(), -3.0);
    }

    #[test]
    fn test_power_right_assoc() {
        // 幂运算右结合：2^3^2 = 2^9 = 512
        assert_eq!(evaluate("2 ^ 3 ^ 2").unwrap(), 512.0);
    }

    #[test]
    fn test_decimal() {
        assert_eq!(evaluate("0.1 + 0.2").unwrap(), 0.30000000000000004); // 浮点误差
        assert_eq!(evaluate("3.5 * 2").unwrap(), 7.0);
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(evaluate("1 / 0").is_err());
    }

    #[test]
    fn test_syntax_errors() {
        assert!(evaluate("1 +").is_err());
        assert!(evaluate("(1 + 2").is_err()); // 缺右括号
        assert!(evaluate("1 @ 2").is_err()); // 非法字符
        assert!(evaluate("").is_err()); // 空表达式
    }

    #[test]
    fn test_complex_expression() {
        // 2 * (3 + 4)^2 - 6 / 2 = 2*49 - 3 = 95
        assert_eq!(evaluate("2 * (3 + 4) ^ 2 - 6 / 2").unwrap(), 95.0);
    }
}
