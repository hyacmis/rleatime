#![allow(dead_code)]

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, Write};

// 按行读取文件 并返回字符串
pub fn read_file(file_path: &String) -> Vec<String> {
    let f = File::open(file_path).expect("未找到文件,请确认文件地址");
    let bf = BufReader::new(f);
    // 处理每一行中的错误, 将其转为String
    // lines 返回的是iter
    let lines: Vec<String> = bf.lines().map(|l| l.unwrap()).collect();
    lines
}

pub enum WriteMode {
    Append,
    Overwrite,
}

pub fn write_file(mode: WriteMode, path: &str, data: &str) -> Result<(), Error> {
    let mut file = match mode {
        WriteMode::Append => OpenOptions::new().append(true).open(path)?,
        WriteMode::Overwrite => File::create(path)?,
    };

    file.write_all(data.as_bytes())
}

#[cfg(test)]
mod test_file {
    use super::*;
    use std::fs;
    use std::io::{Read, Seek, SeekFrom};
    use std::path::Path;

    #[test]
    fn test_create_dir() {
        fs::create_dir("src/file/.tmp").unwrap();
        let is_path = Path::new("src/file/.tmp").exists();
        assert!(is_path);
    }

    #[test]
    fn test_delete_dir() {
        let path = "src/file/.tmp";
        if Path::new(path).exists() {
            // 删除空文件夹
            fs::remove_dir(path).unwrap();
            // 删除不是空的文件夹
            // fs::remove_dir_all(path).unwrap();
        }
        assert!(!Path::new(path).exists());
    }

    #[test]
    fn test_for_dir() {
        let file_names: Vec<&str> = vec![
            "files.rs",
            "csv.rs",
            "test_csv.csv",
            "test_txt.txt",
            "paths.rs",
            "mod.rs",
        ];
        let dir = fs::read_dir("src/file").unwrap();
        for (idx, f) in dir.enumerate() {
            let f = f.unwrap();
            assert_eq!(f.file_name().to_str().unwrap(), file_names[idx]);
        }
    }
    #[test]
    /// 与下面的测试函数一起适用与web开发中的小文件读取
    fn test_read_to_string() {
        // 一次性读取文件的全部内容
        let text = fs::read_to_string("src/file/test_txt.txt").unwrap();
        assert_eq!(text, "h\ne\nl\nl\no\n,\nw\no\nr\nl\nd\n");
    }

    #[test]
    fn test_read_to_u8() {
        // 一次性读取 u8 类型集合
        let text = fs::read("src/file/test_txt.txt").unwrap();
        assert_eq!(
            text,
            vec![
                104, 10, 101, 10, 108, 10, 108, 10, 111, 10, 44, 10, 119, 10, 111, 10, 114, 10,
                108, 10, 100, 10,
            ]
        );
    }

    #[test]
    /// 流式读取
    fn test_read_stream() {
        let mut file = File::open("src/file/test_txt.txt").unwrap();
        // 声明一个缓冲区
        let mut buffer = vec![0; 6];
        file.read_exact(&mut buffer).unwrap();
        assert_eq!(buffer, vec![104, 10, 101, 10, 108, 10]);

        file.read_exact(&mut buffer).unwrap();
        assert_eq!(buffer, vec![108, 10, 111, 10, 44, 10]);
    }

    #[test]
    fn test_read_other() {
        let mut file = File::open("src/file/test_txt.txt").unwrap();
        // 第一种
        // 将游标前移 10 个字节（游标的默认位置是 0）
        file.seek(SeekFrom::Start(10)).unwrap();

        // 将前 10 个字节之后的内容读取到 Buf 中
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        assert_eq!(buffer, ",\nw\no\nr\nl\nd\n");

        // 第二种
        let file2 = File::open("src/file/test_txt.txt").unwrap();
        // 限制读取长度最多为 5 字节
        let mut handle = file2.take(6);
        let mut buffer = vec![0; 5]; // 缓存需要小于等于限制长度, 不然会err

        handle.read_exact(&mut buffer).unwrap();
        assert_eq!(buffer, vec![104, 10, 101, 10, 108]);
    }

    #[test]
    fn test_read_file() {
        let path = "src/file/test_txt.txt".to_string();
        let lines = read_file(&path);
        let expected_lines = ["h", "e", "l", "l", "o", ",", "w", "o", "r", "l", "d"];
        assert_eq!(lines, expected_lines);
    }

    #[test]
    fn test_write_fs() {
        // 一次性写入请谨慎使用！因为它会直接删除文件内容（无论文件多么大）。如果文件不存在就会创建文件
        let path = "src/file/tmp.txt";

        fs::write(path, "FROM RUST PROGRAM").unwrap();

        assert!(Path::new(path).exists());
        fs::remove_file(path).unwrap()
    }

    #[test]
    fn test_write_create() {
        let path = "src/file/tmp.txt";

        let mut file = File::create(path).unwrap();
        file.write_all(b"FROM RUST PROGRAM").unwrap();

        assert_eq!(fs::read_to_string(path).unwrap(), "FROM RUST PROGRAM");

        fs::remove_file(path).unwrap()
    }

    #[test]
    fn test_write_append() {
        let path = "src/file/tmp.txt";
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();

        file.write_all(b" APPEND WORD").unwrap();
        file.write_all(b"\n apple").unwrap();
        assert_eq!(fs::read_to_string(path).unwrap(), " APPEND WORD\n apple");
        fs::remove_file(path).unwrap()
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
