// pub fn test_dny_text(vec_list: &mut Vec<&str>) {
//     for i in vec_list {
//         // 将引用字符串转换为整数字符串，再 +1
//         let mut s: i8 = i.to_string().parse::<i8>().unwrap();
//         s += 1;s
//         println!("{}", s);

//         // 替换原有的值
//         let s1: String = format!("{s}");
//         *i = s1.as_str();
//     }
//     println!("{:?}", vec_list);
// }
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
}
