// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut res = 42;
    let option = Some(12);
    /*
    clippy 提供更好的写法建议，对于设置成 Some(12) 类型的数据，最好对 Some 或者 None 做一个判断
     */
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
