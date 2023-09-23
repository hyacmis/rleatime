#![allow(dead_code)]

use std::{fmt, fs::File, io};

// AppError 是自定义错误类型，它可以是当前包中定义的任何类型，在这里为了简化，我们使用了单元结构体作为例子。
// 为 AppError 自动派生 Debug 特征
pub struct AppError {
    code: usize,
    message: String,
    kind: String, // 错误类型F
}

// 根据错误码显示不同的错误信息
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            404 => "Sorry, Can not find the Page!",
            _ => "Sorry, something is wrong! Please Try Again!",
        };

        write!(f, "{}", err_msg)
    }
}
// 为 AppError 实现 std::convert::From 特征，由于 From 包含在 std::prelude 中，因此可以直接简化引入。
// 实现 From<io::Error> 意味着我们可以将 io::Error 错误转换成自定义的 AppError 错误
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError { kind: String::from("io"), message: error.to_string(), code: 500 }
    }
}
// 手动指定打印类型
impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AppError {{ code: {}, message: {} ,kind:{}}}",
            self.code, self.message, self.kind
        )
    }
}

// 一个示例函数用于产生 AppError 错误
fn produce_error() -> Result<(), AppError> {
    Err(AppError { code: 404, message: String::from("Page not found"), kind: "main".to_string() })
}

pub fn main() -> Result<(), AppError> {
    match produce_error() {
        Err(e) => eprintln!("{}", e),
        _ => println!("No error"),
    }
    eprintln!("{:?}", produce_error());

    eprintln!("{:#?}", produce_error());
    let _file = File::open("nonexistent_file.txt")?;

    Ok(())
}
