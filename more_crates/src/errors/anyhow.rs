use std::fs;

use anyhow::{Context, Result};

/// 直接返回原始的error
pub fn read_file_origin_error() -> Result<String> {
    let data = fs::read_to_string("file.txt")?;
    Ok(data)
}

/// 使用 anyhow 的 Result 返回error
pub fn read_file_anyhow_error(file_path: &str) -> Result<String> {
    // 尝试读取文件
    let data = fs::read_to_string(file_path)
        // 使用 with_context 来附加错误信息，然后利用 ? 语法糖传播错误。
        .with_context(|| format!("failed to read file `{}`", file_path))?;

    Ok(data)
}

// 自定义Error,
// 通过#[derive(Debug)]注解实现std::fmt::Debug的trait
#[derive(Debug)]
pub struct CustomError {
    err: ChildError,
}

// 实现Display的trait
impl std::fmt::Display for CustomError {
    // 一般情况下是固定写法
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "父类型错误~")
    }
}

// 实现std::error::Error Trait,因为有子Error:ChildError,覆盖source()方法,返回Some(err)
impl std::error::Error for CustomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.err)
    }
}

// 子Error
#[derive(Debug)]
struct ChildError;

// 实现Display
impl std::fmt::Display for ChildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "子类型错误~")
    }
}

// 实现Error的trait,因为没有子Error,不需要覆盖source()方法
impl std::error::Error for ChildError {}

// 构建一个Result的结果，返回自定义的error:CustomError
pub fn get_super_error() -> Result<(), CustomError> {
    Err(CustomError { err: ChildError })
}

#[cfg(test)]
mod test_anyhow {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_origin_error() {
        let res = read_file_origin_error();

        if let Err(e) = res {
            assert_eq!(e.to_string(), "No such file or directory (os error 2)");
        }
    }

    #[test]
    fn test_open_file_error() {
        let res = read_file_anyhow_error("./anyhow.txt");

        if let Err(e) = res {
            assert_eq!(e.to_string(), "failed to read file `./anyhow.txt`");
        }
    }

    #[test]
    fn test_custom_error() {
        match get_super_error() {
            Err(e) => {
                assert_eq!(e.to_string(), "父类型错误~");
                assert_eq!(e.source().unwrap().to_string(), "子类型错误~");
            }
            _ => println!("No error"),
        }
    }
}
