#![allow(dead_code)]

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, Write};

// 按行读取文件 并返回字符串
fn read_file(file_path: &String) -> Vec<String> {
    let f = File::open(file_path).expect("未找到文件,请确认文件地址");
    let bf = BufReader::new(f);
    // 处理每一行中的错误, 将其转为String
    // lines 返回的是iter
    let lines: Vec<String> = bf.lines().map(|l| l.unwrap()).collect();
    lines
}

enum WriteMode {
    Append,
    Overwrite,
}

fn write_file(mode: WriteMode, path: &str, data: &str) -> Result<(), Error> {
    let mut file = match mode {
        WriteMode::Append => OpenOptions::new().append(true).open(path)?,
        WriteMode::Overwrite => File::create(path)?,
    };

    file.write_all(data.as_bytes())
}

#[cfg(test)]
mod test_file {
    use super::*;

    #[test]
    fn test_read_file() {
        let path = "src/file/test_txt.txt".to_string();
        let lines = read_file(&path);
        let expected_lines = ["h", "e", "l", "l", "o", ",", "w", "o", "r", "l", "d"];
        assert_eq!(lines, expected_lines);
    }

    #[test]
    fn test_write_file() {
        let path = "src/file/test_txt.txt".to_string();
        let txt = "被我删除了啦".to_string();

        let _ = write_file(WriteMode::Overwrite, &path, &txt);

        let f = read_file(&path);
        assert_eq!(f, vec![txt]);

        // 清空文件
        let _ = write_file(WriteMode::Overwrite, &path, "");

        // 需要换行符才能换行,暂时不知道如何写入分隔符
        let expected_lines = [
            "h\n", "e\n", "l\n", "l\n", "o\n", ",\n", "w\n", "o\n", "r\n", "l\n", "d\n",
        ];

        for i in expected_lines {
            let _ = write_file(WriteMode::Append, &path, i);
        }
    }
}
