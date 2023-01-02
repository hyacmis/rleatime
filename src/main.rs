fn test1() {
    let name = "Hello John";
    let slip = name.split_whitespace().collect::<Vec<_>>();

    println!("{slip:?}");
    let mut s1 = Vec::new();

    for idx in 1..6 {
        s1.push(format!("idx_{}", idx));
    }
    println!("{s1:?}");
}

fn main() {
    test1();
}
