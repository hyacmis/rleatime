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
mod test_iter {
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
}
