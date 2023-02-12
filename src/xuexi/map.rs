pub fn mapping() {
    use std::collections::HashMap;

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];
    // 将行值的k v 转为 map的k v 需要手动指定 类型标注
    let teams_map: HashMap<_, _> = teams_list.into_iter().collect();

    println!("{:?}", teams_map);

    let name = String::from("Sunface");
    let age = 18;

    let mut handsome_boys = HashMap::new();
    // handsome_boys.insert(name, age); //因为生命周期的原因,外部的元素已不可访问
    handsome_boys.insert(&name, age); // 传入引用即可

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);
    println!("{:?}", score);
    // 遍历 map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();

    scores.insert("Blue", 10);
    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));

    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入
    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    // 根据空格来切分字符串(英文单词都是通过空格切分)
    for i in text.split_whitespace() {
        let count = map.entry(i).or_insert(0);
        *count += 1;
        // or_insert 返回了 &mut v 引用，因此可以通过该可变引用直接修改 map 中对应的值
        // 使用 count 引用时，需要先进行解引用 *count，否则会出现类型不匹配
    }
    println!("{:?}", map);

    println!("{:?}", map.keys());
    println!("{:?}", map.values());

}
