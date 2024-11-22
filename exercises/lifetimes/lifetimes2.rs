// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
一个观点:
生命周期的概念是可以直接通过 scope 和所有权得到的，
在C等内存会泄漏的语言，存在指向失效数据数据指针的悬垂指针问题，是因为没有规定离开作用域后自动调用 Drop 销毁内存
如果函数特别地返回了引用，需要确保引用指向的数据有效，显式说明生命周期( 一步一步推导出来的 )

原来的写法:
let result;
{
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
}

result 变量在外部作用域中声明，在内部作用域中被赋值。
由于内部作用域中的 string2 在作用域结束时会被销毁，意味着 result 可能会持有对已经无效的内存的引用，导致未定义行为。
*/
fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;
    {
        // let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}