#![allow(dead_code)]

/// 一个简易的数学表达式的分词器
/// 拆分带有括号和负数

#[derive(Debug, PartialEq)]
pub enum Token {
    Num(f64),
    Op(char),
    Paren(char),
}

fn flush_buf(tokens: &mut Vec<Token>, number_buf: &mut String) {
    if !number_buf.is_empty() {
        tokens.push(Token::Num(number_buf.parse().unwrap()));
        number_buf.clear();
    }
}

/// 分割字符串
pub fn tokenization(expression: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buf = String::new();
    // 记录前一个字符
    let mut prev_char: Option<char> = None;

    for c in expression.chars() {
        match c {
            '0'..='9' | '.' => buf.push(c),
            '-' => {
                // 需要判断负数, 规则:
                // 1）若前一个字符为‘('，则必定为负号；
                // 2）若前面一个字符为其他运算符，如*，/，则必定是负号；
                // 3）若前一个字符为')'或者数字，则必定为减号；
                // 4）若前面没有字符，即该字符为表达式的第一个字符，则必定是减号。

                // 为减号
                if prev_char.is_some() // 前一个字符存在
                    && !matches!( // 前一个字符不为 运算符和`(` 即 前一个字符时数字和`)`
                        prev_char,
                        Some('+') | Some('-') | Some('*') | Some('/') | Some('(')
                    )
                {
                    flush_buf(&mut tokens, &mut buf);
                    tokens.push(Token::Op(c));
                } else {
                    // 为负号 在数字前
                    buf.push(c);
                }
            }
            '+' | '*' | '/' => {
                // buf缓冲区不为空时
                flush_buf(&mut tokens, &mut buf);
                tokens.push(Token::Op(c));
            }
            '(' | ')' => {
                flush_buf(&mut tokens, &mut buf);
                tokens.push(Token::Paren(c))
            }
            _ => {}
        }
        prev_char = Some(c)
    }
    flush_buf(&mut tokens, &mut buf);
    tokens
}

#[cfg(test)]
mod test_tokenizer {
    use super::*;

    #[test]
    fn test_token_ordinary() {
        let expression = "1+3*(4-2)/2";
        let res = tokenization(expression);
        assert_eq!(
            res,
            vec![
                Token::Num(1.0),
                Token::Op('+'),
                Token::Num(3.0),
                Token::Op('*'),
                Token::Paren('('),
                Token::Num(4.0),
                Token::Op('-'),
                Token::Num(2.0),
                Token::Paren(')'),
                Token::Op('/'),
                Token::Num(2.0),
            ]
        )
    }

    #[test]
    fn test_token_with_start_negative_numbers() {
        let expression = "-1.5+4.5/2-(2/2)";
        let res = tokenization(expression);
        assert_eq!(
            res,
            vec![
                Token::Num(-1.5),
                Token::Op('+'),
                Token::Num(4.5),
                Token::Op('/'),
                Token::Num(2.0),
                Token::Op('-'),
                Token::Paren('('),
                Token::Num(2.0),
                Token::Op('/'),
                Token::Num(2.0),
                Token::Paren(')'),
            ]
        )
    }

    #[test]
    fn test_token_parentheses() {
        let expression = "(1+2)*(3-4)";
        let res = tokenization(expression);
        assert_eq!(
            res,
            vec![
                Token::Paren('('),
                Token::Num(1.0),
                Token::Op('+'),
                Token::Num(2.0),
                Token::Paren(')'),
                Token::Op('*'),
                Token::Paren('('),
                Token::Num(3.0),
                Token::Op('-'),
                Token::Num(4.0),
                Token::Paren(')'),
            ]
        )
    }

    #[test]
    fn test_token_nested_parentheses() {
        let expression = "((1+2)*3-(-4/-2))";
        let res = tokenization(expression);
        assert_eq!(
            res,
            vec![
                Token::Paren('('),
                Token::Paren('('),
                Token::Num(1.0),
                Token::Op('+'),
                Token::Num(2.0),
                Token::Paren(')'),
                Token::Op('*'),
                Token::Num(3.0),
                Token::Op('-'),
                Token::Paren('('),
                Token::Num(-4.0),
                Token::Op('/'),
                Token::Num(-2.0),
                Token::Paren(')'),
                Token::Paren(')'),
            ]
        )
    }

    #[test]
    fn test_token_empty_parentheses() {
        let expression = "()";
        let res = tokenization(expression);
        assert_eq!(res, vec![Token::Paren('('), Token::Paren(')'),])
    }

    #[test]
    fn test_token_parentheses_and_operators() {
        let expression = "(-1+2)*3/(4-2)";
        let res = tokenization(expression);
        assert_eq!(
            res,
            vec![
                Token::Paren('('),
                Token::Num(-1.0),
                Token::Op('+'),
                Token::Num(2.0),
                Token::Paren(')'),
                Token::Op('*'),
                Token::Num(3.0),
                Token::Op('/'),
                Token::Paren('('),
                Token::Num(4.0),
                Token::Op('-'),
                Token::Num(2.0),
                Token::Paren(')'),
            ]
        )
    }

    #[test]
    fn test_token_parentheses_and_negative_numbers() {
        let expression = "(-1+2)*(-3/4)";
        let res = tokenization(expression);
        assert_eq!(
            res,
            vec![
                Token::Paren('('),
                Token::Num(-1.0),
                Token::Op('+'),
                Token::Num(2.0),
                Token::Paren(')'),
                Token::Op('*'),
                Token::Paren('('),
                Token::Num(-3.0),
                Token::Op('/'),
                Token::Num(4.0),
                Token::Paren(')'),
            ]
        )
    }

    #[test]
    fn test_extra_parentheses() {
        let expression = "-1+2*(-3)";
        let res = tokenization(expression);

        assert_eq!(
            res,
            vec![
                Token::Num(-1.0),
                Token::Op('+'),
                Token::Num(2.0),
                Token::Op('*'),
                Token::Paren('('),
                Token::Num(-3.0),
                Token::Paren(')'),
            ]
        )
    }
}
