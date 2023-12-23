#![allow(dead_code)]

pub fn main_iter() {
    let values = vec![1, 2, 3];

    for v in values.into_iter() {
        println!("{}", v)
    }

    // 下面的代码将报错，因为 values 的所有权在上面 `for` 循环中已经被转移走
    // println!("{:?}",values);

    let values = vec![1, 2, 3];
    let _values_iter = values.iter();

    // 不会报错，因为 values_iter 只是借用了 values 中的元素
    println!("{:?}", values);

    let mut values = vec![1, 2, 3];
    // 对 values 中的元素进行可变借用
    let mut values_iter_mut = values.iter_mut();

    // 取出第一个元素，并修改为0
    if let Some(v) = values_iter_mut.next() {
        *v = 0;
    }

    // 输出[0, 2, 3]
    println!("{:?}", values);
}

#[cfg(test)]
mod test_vec {
    #[test]
    fn test1() {
        let name = "Hello John";
        let slip = name.split_whitespace().collect::<Vec<_>>();

        println!("{slip:?}");
        let mut s1 = Vec::new();

        for idx in 1..6 {
            s1.push(format!("idx_{idx}"));
        }
        println!("{s1:?}");

        for i in &s1 {
            println!("{i}")
        }
    }

    #[test]
    fn test_text2() {
        // for 循环中动态添加值
        let mut result = Vec::new();
        for i in 0..10 {
            result.push(format!("string{i}"));
        }
        result[2] = "string_new".to_string();
        println!("{:?}", result);
    }

    #[test]
    fn test_text3() {
        let mut vec_list: Vec<&str> = vec!["1", "2", "3", "4", "5"];

        let v = "";

        for i in vec_list.iter_mut() {
            *i = v;
        }
        assert_eq!(vec_list, vec!["", "", "", "", ""]);
    }

    #[test]
    fn test_dny_text() {
        let mut vec_list: Vec<String> = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
            String::from("6"),
            String::from("7"),
            String::from("8"),
            String::from("9"),
        ];

        for i in vec_list.iter_mut() {
            // 将引用字符串转换为整数字符串，再 +1
            let s: i8 = i.parse::<i8>().unwrap() + 1;
            // 解引用 需要存在引用才能解引用
            *i = format!("{s}");
        }
        assert_eq!(vec_list, vec!["2", "3", "4", "5", "6", "7", "8", "9", "10"]);

        for i in &mut vec_list {
            // 替换原有的值
            *i = format!("arr_{i}");
        }
        assert_eq!(
            vec_list,
            vec!["arr_2", "arr_3", "arr_4", "arr_5", "arr_6", "arr_7", "arr_8", "arr_9", "arr_10"]
        );
        // 所有权移交
        for mut i in vec_list {
            // 在拥有所有权的时候遍历,不需要解引用 可直接修改元素本身
            i = format!("{i}_move");
            println!("{i}");
        }
        // 后续无法再使用 vec_list
    }
}
