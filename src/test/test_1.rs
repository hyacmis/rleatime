#[cfg(test)]
mod testss {
    use crate::xuexi::r#box::foo;

    #[test]
    fn test_foo() {
        let b = foo("world");
        assert_eq!(b, "Hello, world");
    }
}
