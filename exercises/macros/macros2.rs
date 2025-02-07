// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.

// 这个程序主要想说，rust 里面函数可以定义在 fn main() 之后，但是对于宏编程而言，需要在使用之前找到
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}


