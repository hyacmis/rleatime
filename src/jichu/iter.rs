#![allow(dead_code)]

fn iter_update(mut list: Vec<i32>) -> Vec<i32> {
    for v in list.iter_mut() {
        // 解引用 通过指针修改值
        *v += 1
    }
    list
}

fn iter_life(list: Vec<i32>) -> Vec<i32> {
    let mut arr = Vec::new();
    for v in list.iter() {
        arr.push(*v);
    }
    arr
}

#[cfg(test)]
mod test_vec {
    use super::*;

    #[test]
    fn test_update() {
        let arr = vec![1, 2, 3, 4, 5];
        let res = iter_update(arr);
        assert_eq!(res, vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_collect() {
        //测试collect 的收集
        let name = "Hello John";
        let slip = name.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(slip, vec!["Hello", "John"]);
    }

    #[test]
    fn test_for_push() {
        // for 循环中动态添加值
        let mut result = Vec::new();
        for i in 0..5 {
            result.push(format!("string{i}"));
        }
        result[2] = "string_new".to_string();
        assert_eq!(
            result,
            vec!["string0", "string1", "string_new", "string3", "string4"]
        );
    }
}
