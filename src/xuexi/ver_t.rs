#![allow(dead_code)]
pub fn v() {
    let mut v = vec![1];
    // 添加
    v.push(1);
    println!("{v:?}");

    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("第三个元素是 {third}");

    match v.get(2) {
        Some(third) => println!("第三个元素是 {third}"),
        None => println!("去你的第三个元素，根本没有！"),
    }
    // 数组越界会报错
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    println!("{does_not_exist:?}");

    // 遍历数组
    for i in &v {
        println!("{i}");
    }
    // 遍历时修改
    for i in &mut v {
        *i += 10 // 解引用
    }
    // 同时获取数组的下标和值  注意v的权限
    for (i, v) in v.iter().enumerate() {
        println!("{i},{v}");
    }
    println!("{v:?}")
}

