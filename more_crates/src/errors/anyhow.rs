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

#[cfg(test)]
mod test_anyhow {

    use super::*;

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
            // 使用 downcast 来反解出实际的错误实例，本案例中可能出现的异常是 io::Error。
            assert_eq!(e.to_string(), "failed to read file `./anyhow.txt`");
        }
    }
}
