#![allow(dead_code)]

use std::ops::Deref;

/// 使用场景:
/// 1. 特意的将数据分配在堆上
/// 2. 数据较大时，又不想在转移所有权时进行数据拷贝
/// 3. 类型的大小在编译期无法确定，但是我们又需要固定大小的类型时
/// 4. 特征对象，用于说明对象实现了一个特征，而不是某个特定的类型

/// DST 到 Sized
fn dst_to_sized() {
    //会出现下面的报错:
    //recursive type `senior::smart_pointer::boxes::test_box::test_dyn_size::List` has infinite size [E0072]
    // Help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
    enum List {
        Cons(i32, Box<List>), // 使用 `Box` 即可将其转为固定大小
        Nil,
    }
}

/// 使用场景:
/// 需要一个在运行期初始化的值，但是可以全局有效，也就是和整个程序活得一样久
/// Rc/Arc 也可以实现此功能，但是 Box::leak 是性能最高的。
fn gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("hello, world");

    // Box::leak 我们不仅返回了一个 &str 字符串切片，它还是 'static 生命周期的！
    Box::leak(s.into_boxed_str())
}

#[cfg(test)]
mod test_box {
    use super::gen_static_str;

    #[test]
    fn test_deref() {
        //存储到堆上
        let x = Box::new(1);
        // 需要使用解引用 `显示` 的对其操作
        let y = *x + 2;
        assert_eq!(y, 3);
    }

    #[test]
    fn test_ownership() {
        // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
        let arr = Box::new([0; 1000]);
        // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
        // let arr1 = arr;
        // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
        // assert_eq!(arr.len(), 1000);
        assert_eq!(arr.len(), 1000);
        // 使用clone() 才能将底层数据转移
        let arr1 = arr.clone();
        assert_eq!(arr1.len(), 1000);
    }

    /// 在 Rust 中，想实现不同类型组成的数组只有两个办法：
    /// 枚举: 限制较多
    /// 特征对象: 往往是最常用的解决办法。
    #[test]
    fn test_trait_object() {
        trait Draw {
            fn draw(&self) -> String;
        }

        struct Button {
            id: u32,
        }
        impl Draw for Button {
            fn draw(&self) -> String {
                format!("这是屏幕上第{}号按钮", self.id)
            }
        }

        struct Select {
            id: u32,
        }

        impl Draw for Select {
            fn draw(&self) -> String {
                format!("这个选择框贼难用{}", self.id)
            }
        }

        //Box<dyn Draw> 就是特征对象
        //可以理解为实现了该 trait 的结构体/对象
        let elems: Vec<Box<dyn Draw>> =
            vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];

        let res = elems.iter().map(|x| x.draw()).collect::<Vec<String>>();

        assert_eq!(
            res,
            [
                "这是屏幕上第1号按钮".to_string(),
                "这个选择框贼难用2".to_string()
            ]
        )
    }

    #[test]
    fn test_leak() {
        let s = gen_static_str();
        assert_eq!(s, "hello, world")
    }
}

///定义自己的智能指针
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    /// Rust 会在解引用时自动把智能指针和 &&&&v 做引用归一化操作，转换成 &v 形式，最终再对 &v 进行解引用
    /// 赋值操作需要手动解引用
    /// 方法调用会自动解引用
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test_my_box {
    use super::MyBox;

    #[test]
    fn my_box() {
        let y = MyBox::new(5);
        //如果未使用解引用会出现下面的报错:
        //type `senior::smart_pointer::boxes::MyBox<{integer}>` cannot be dereferenced [E0614]
        assert_eq!(5, *y);
    }
}
