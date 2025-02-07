// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

/*
最终的修改是在不同的分支之间添加 ;
macro_rules! 定义的宏分支之间缺少了分号或逗号来分隔不同的规则。
每个宏规则（不同的模式)之间需要使用分号或逗号进行分隔。
表示不同的匹配模式，从而编译器明确规则之间的界限。
my_macro!() 和  my_macro!(7777) 都会正确匹配对应的宏分支并输出预期的结果

*/
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}