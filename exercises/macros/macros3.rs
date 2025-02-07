// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

/*
这里考察的问题是，对于宏定义，这里的宏 my_macro 定义在模块 macros 内，
但是在 main 函数中使用宏 my_macro!() 时，没有导入这个宏。
在 rust 中，即使宏是在同一个 crate 中定义的，如果它们位于不同的模块内，也需要通过 use 语句将其引入当前作用域，或者直接使用
完整路径来调用。
1. use macros::my_macro;

2. macros::my_macro!();

直接使用 1 会导致错误，报错信息显示 no `my_macro` in `macros`
Rust 的模块系统(mod) 与宏的定义和作用域之间的交互方式。
当在 Rust 中定义宏时，它们的作用域和其他类型的项(如函数和类型)的作用域不同。
宏在其定义的模块中是私有的，除非明确地公开它们。

修改: 
1. 宏默认是私有的。即使在同一个模块中，你仍然需要显式地将宏定义为公有(public)以便在其他地方使用
修改方式可以在 macro_rules! my_macro 前面加上 #[macro_export] 属性，表示将宏导出到 crate 的根作用域。
*/

mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}