// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.


trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
/*
[psNote]: 一个错误说明
之前的写法就是
impl AppendBar for Vec<String>{
    fn append_bar(self) -> Self {
        self.push(String::from("Bar"))
    }
}

直接查看 push 的返回值，发现 push 返回类型是 (), 实际不返回值
显式地返回被修改过的向量 `self`, 定义类型为 mut self
因为 push 方法会改变向量的状态，最后返回 self 遵循方法的签名要求，确保返回的类型是 `Self`
*/
impl AppendBar for Vec<String>{
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}