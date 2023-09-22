#[cfg(test)]
mod testss {
    use crate::xuexi::gaoji::r#box;

    #[test]
    fn test_foo() {
        let b = r#box::foo("world");
        assert_eq!(b, "Hello, world");
    }
    #[test]
    fn test_fn1() {
        r#box::fn1()
    }
}
