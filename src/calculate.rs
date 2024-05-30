#![allow(dead_code)]
use crate::tokenizer::{tokenization, Token};

fn calculate(expression: &str) -> Result<f64, &'static str> {
    let tokens = tokenization(expression);
    let mut stack = Vec::new();
    let mut op_stack = Vec::new();

    for t in tokens {
        match t {
            Token::Num(num) => stack.push(num),
            Token::Op(op) => {
                while let Some(&top_op_char) = op_stack.last() {
                    if !op_stack.is_empty() && precedence(op) <= precedence(top_op_char) {
                        let top_op = op_stack.pop().expect("op missing");
                        let right = stack.pop().expect("num missing");
                        let left = stack.pop().expect("num missing");
                        stack.push(apply_operator(top_op, left, right));
                    } else {
                        break;
                    }
                }
                op_stack.push(op);
            }
            Token::Paren(paren) => match paren {
                '(' => op_stack.push(paren),
                ')' => {
                    while let Some(&top_op) = op_stack.last() {
                        if top_op == '(' {
                            op_stack.pop();
                            break;
                        } else {
                            let right = stack.pop().expect("num missing");
                            let left = stack.pop().expect("num missing");
                            let op = op_stack.pop().expect("op missing");
                            stack.push(apply_operator(op, left, right));
                        }
                    }
                }
                _ => return Err("Invalid parentheses"),
            },
        }
    }

    while let Some(op_char) = op_stack.pop() {
        if op_char != '(' {
            let right = stack.pop().expect("num missing");
            let left = stack.pop().expect("num missing");
            stack.push(apply_operator(op_char, left, right));
        }
    }

    Ok(stack.pop().expect("calculate error"))
}

fn precedence(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn apply_operator(op: char, left: f64, right: f64) -> f64 {
    match op {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => left / right,
        _ => panic!("Invalid operator"),
    }
}

#[cfg(test)]
mod test_calc {
    use super::*;

    #[test]
    fn test_calc_simple_expression() {
        let expression = "1+2*3";
        let result = calculate(expression);
        assert_eq!(result, Ok(7.0));
    }

    #[test]
    fn test_calc_expression_with_parentheses() {
        let expression = "(1+2)*3";
        let result = calculate(expression);
        assert_eq!(result, Ok(9.0));
    }

    #[test]
    fn test_calc_expression_with_negative_numbers() {
        let expression = "-1+2*(-3)";
        let result = calculate(expression);
        assert_eq!(result, Ok(-7.0));
    }

    #[test]
    fn test_calc_expression_with_float_numbers() {
        let expression = "1.5+2.5*3.5";
        let result = calculate(expression);
        assert_eq!(result, Ok(10.25));
    }

    #[test]
    #[should_panic]
    fn test_calc_invalid_expression() {
        let expression = "1+2*";
        let _ = calculate(expression);
    }

    #[test]
    fn test_calc_mismatched_parentheses() {
        let expression = "(1+2)*3)";
        let result = calculate(expression);
        assert_eq!(result, Ok(9.0));
    }
}
