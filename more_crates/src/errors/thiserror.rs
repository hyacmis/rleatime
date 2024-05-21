use std::{fs, io};

use thiserror::Error;

use super::anyhow::{ChildError, CustomError};

// 自定义错误类型的定义
#[derive(Error, Debug)]
pub enum MyError {
    #[error("data not found")]
    DataNotFound,
    #[error("invalid input")]
    InvalidInput,
    #[error("I/O error occurred")]
    IoError(#[from] io::Error),
    #[error("failed with code: {0}")] //其中 {0} 会被动态地替换为具体的代码值
    FailedWithCode(i32),
}

// 模拟从其他库中导入的错误类型
// 自定义错误类型的定义
#[derive(Error, Debug)]
pub enum OnceError {
    // OtherError 的描述，它直接从其内部的错误类型继承
    #[error(transparent)] // 该错误只是作为其他错误的容器，它的错误消息将直接从其“源”错误中继承。
    OtherError(#[from] CustomError),
}

// 示例函数，展示如何使用自定义错误
pub fn search_data(query: &str) -> Result<(), MyError> {
    if query.is_empty() {
        return Err(MyError::InvalidInput);
    }
    let data = fs::read_to_string(query).map_err(MyError::from)?;
    if !data.contains(query) {
        return Err(MyError::DataNotFound);
    }
    Ok(())
}

// 示例函数，展示如何使用动态错误消息
pub fn process_data() -> Result<(), MyError> {
    let error_code = 404;
    // 使用动态的 error_code 创建 FailedWithCode 错误
    Err(MyError::FailedWithCode(error_code))
}

// 示例函数，展示如何从其他错误类型转换
pub fn interface_with_other_lib() -> Result<(), OnceError> {
    // 调用其他库的函数...
    // 如果那个函数返回了一个错误，我们使用 MyError::from 将它转换为 MyError::OtherError
    Err(OnceError::from(CustomError { err: ChildError }))
}

#[cfg(test)]
mod test_thiserror {
    use super::*;

    #[test]
    fn test_search_data() {
        let res = search_data("haha");
        if let Err(e) = res {
            match e {
                MyError::DataNotFound => assert_eq!(e.to_string(), "data not found"),
                MyError::InvalidInput => assert_eq!(e.to_string(), "invalid input"),
                MyError::IoError(_) => {
                    assert_eq!(e.to_string(), "I/O error occurred")
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_dyn_error() {
        let res = process_data();
        if let Err(MyError::FailedWithCode(i)) = res {
            assert_eq!(i, 404)
        }
    }

    #[test]
    fn test_conversion_error() {
        let res = interface_with_other_lib();
        if let Err(OnceError::OtherError(e)) = res {
            assert_eq!(e.to_string(), "父类型错误~")
        }
    }
}
