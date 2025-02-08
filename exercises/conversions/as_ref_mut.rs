// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
/*
回顾之前 impl [trait X] for [type Y] 的语法
指的是为某一种类型 Y 实现特征 X,
在我过去的经验里，type Y 使用泛型参数来表示比较常见，用例是:
struct MyGenericStruct<T>{
    value: T
}
impl<T: Clone> MyTrait for MyGenericStruct<T> {}
// 泛型结构体
而之前的 [trait X] 部分，也需要表明传入的参数是哪一种类型,
trait MyTrait<T> {
    fn my_method(&self, item: T);
}
指定对应 trait 中的方法接收哪种类型的参数
+ AsRef<str>: 这里的 <str> 是 AsRef trait 的泛型参数，指明了想要实现的 trait 是针对 str 类型的引用
实现了 AsRef<str> 的类型可以通过调用 .as_ref() 获得一个 &str 类型的引用
+ AsRef 本身是一个泛型 trait(泛型特征)，需要指定它的泛型参数类型。
如果只写了 impl AsRef for T{}, 编译器不知道你想要把 T 转换成什么类型的引用，因为没有指定泛型参数。
AsRef 需要知道你要"借用"成哪种类型，必须提供具体的类型参数。
*/

// Obtain the number of bytes (not characters) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.


fn byte_counter<T>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn char_counter<T>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using as_mut().
// TODO: Add the appropriate trait bound.
fn num_sq<T>(arg: &mut T) {
    // TODO: Implement the function body.
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
