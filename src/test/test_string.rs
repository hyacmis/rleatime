#[cfg(test)]
mod testss {
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
        let mut vec_list = vec![
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
        println!("{vec_list:?}");
        for i in vec_list.iter_mut() {
            // 将引用字符串转换为整数字符串，再 +1
            let s: i8 = i.parse::<i8>().unwrap() + 1;

            // 替换原有的值
            *i = format!("{s}");
        }
        println!("{:?}", vec_list);

        /*
            解引用 需要存在引用才能解引用
            在拥有所有权的时候遍历,不需要解引用 可直接修改元素本身
        */
        // for mut i in vec_list{
        //     // 将引用字符串转换为整数字符串，再 +1
        //     let s: i8 = i.parse::<i8>().unwrap() + 1;
        //     i = format!("{s}");
        //     println!("{:?}", i);
        //     // 替换原有的值
        //     // *i = format!("{s}");
        // }
    }
    #[test]
    fn test_dny_text_2() {
        let mut vec_list = vec![
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
        println!("{vec_list:?}");
        for i in &mut vec_list {
            // 将引用字符串转换为整数字符串，再 +1
            let s: i8 = i.to_string().parse::<i8>().unwrap() + 1;

            // 替换原有的值
            *i = format!("{s}");
        }
        println!("{:?}", vec_list);
    }

    #[test]
    fn test_dny_text_3() {
        let mut vec_list = vec![
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
        println!("{vec_list:?}");
        for i in &mut vec_list {
            // 替换原有的值
            *i = format!("arr_{i}");
        }
        println!("{:?}", vec_list);
    }
}
