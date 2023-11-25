#![allow(dead_code)]

use std::fs::File;
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

#[test]
fn test_read_file() {
    let path = "src/xuexi/jinjie/test_txt.txt".to_string();
    let lines = read_file(&path);
    let expected_lines = [
        "max_width = 100                   # 设置最大行宽为 100 个字符",
        "tab_spaces = 4                    # 设置缩进宽度为 4 个空格",
        "edition = \"2021\"                  # 设置 Rust 版本（根据实际项目版本进行调整）",
        "use_small_heuristics = \"Max\"      # 设置换行策略",
        "newline_style = \"Auto\"            # 设置换行符风格，根据平台自动选择",
    ];
    assert_eq!(lines, expected_lines);
}

fn write_file(file_path: &String, txt: &String) -> Result<(), Error> {
    let mut output = File::create(file_path)?;
    File::write_all(&mut output, txt.as_ref())?;
    Ok(())
}

#[test]
fn test_write_file() {
    let path = "src/xuexi/jinjie/test_txt.txt".to_string();
    let txt = "被我删除了啦".to_string();

    let _ = write_file(&path, &txt);

    let f = read_file(&path);
    assert_eq!(f, vec![txt]);
}
