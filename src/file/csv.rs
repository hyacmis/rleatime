#![allow(dead_code)]

use std::error::Error;
use std::fs::File;

use csv::{Reader, ReaderBuilder};
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Serialize)]
struct Record {
    year: u16,
    make: String,
    model: String,
    description: String,
}

#[cfg(test)]
mod test_csv {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_read_csv() {
        let path = "src/file/test_csv.csv".to_string();
        let mut records = read_csv(&path, b',', true).unwrap();

        let handle = records.headers().cloned().unwrap();

        let mut arr = Vec::new();

        for result in records.records() {
            let record = result.unwrap();
            // 通过handel 获取到列名对应的值
            let col = handle.iter().position(|x| x == "year").unwrap();
            let value = record.get(col).unwrap().to_owned();
            arr.push(value);
        }
        assert_eq!(arr, vec!["1948", "1967"])
    }

    #[test]
    fn test_read_csv_serde() {
        let path = "src/file/test_csv.csv".to_string();
        let mut reader = read_csv(&path, b',', true).unwrap();
        // deserialize方法 解析至 Record struct

        let dict = HashMap::from([
            (
                0,
                Record {
                    year: 1948,
                    make: "Porsche".to_string(),
                    model: "356".to_string(),
                    description: "Luxury sports car".to_string(),
                },
            ),
            (
                1,
                Record {
                    year: 1967,
                    make: "Ford".to_string(),
                    model: "Mustang fastback 1967".to_string(),
                    description: "American car".to_string(),
                },
            ),
        ]);

        for (idx, record) in reader.deserialize().enumerate() {
            let record: Record = record.unwrap();
            let res = dict.get(&idx).unwrap();
            assert_eq!(record.year, res.year);
            assert_eq!(record.make, res.make);
            assert_eq!(record.model, res.model);
            assert_eq!(record.description, res.description);
        }
    }
}
