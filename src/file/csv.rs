#![allow(dead_code)]

use std::error::Error;
use std::fs::File;

use csv;
use csv::{Reader, ReaderBuilder};
use serde::Deserialize;

fn read_csv(
    file_path: &str,
    delimiter: u8,
    has_headers: bool,
) -> Result<Reader<File>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let rdr = ReaderBuilder::new()
        .has_headers(has_headers)
        .delimiter(delimiter)
        .from_reader(file);
    Ok(rdr)
}

#[test]
fn test_read_csv() {
    let path = "src/file/test_csv.csv".to_string();
    let mut records = read_csv(&path, b',', true).unwrap();

    let handle = records.headers().cloned().unwrap();

    for result in records.records() {
        let record = result.unwrap();
        // 通过handel 获取到列名对应的值
        let col = handle.iter().position(|x| x == "year").unwrap();
        println!("{:?}", record.get(col));
        // 可循环打印出列值
        // for i in record.iter() {
        //     println!("{:?}", i)
        // }
        // println!("{:?}", record); // 只能拿到行数据
    }
}

#[derive(Deserialize)]
struct Record {
    year: u16,
    make: String,
    model: String,
    description: String,
}

#[test]
fn test_read_csv_serde() {
    let path = "src/file/test_csv.csv".to_string();
    let mut reader = read_csv(&path, b',', true).unwrap();
    // deserialize方法 解析至 Record struct
    for record in reader.deserialize() {
        let record: Record = record.unwrap();
        println!("{}", record.year);
        println!("{}", record.make);
        println!("{}", record.model);
        println!("{}", record.description);
    }
}
