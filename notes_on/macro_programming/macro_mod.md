# rustlings 中的 macros3.rs 涉及到的相关问题
原来的 macros3.rs 代码如下:
```rust
mod macros{
    macro_rules! my_macro{
        () => {
            println!("Checkout my_macros!");
        }
    }
}

fn main(){
    my_macro!();
}
```
由于直接定义在模块内部，模块内部的宏设计为默认私有，修改方式有两种。
## 1. #[macro_export] 根部导出
> `#[macro_export]` 使宏在定义它的 crate 中可见。这意味着这个宏可以在该 crate 的其他模块中使用，并且可以从该 crate 的外部进行访问。但是，它不会改变宏的导出路径，`#[macro_export]` 标记的宏通常是在 crate 的根进行导出，而不是从定义它的模块导出。

使用 `#[macro_export]` 时，宏被导出到 crate 根，因此你可以直接使用它，而不是通过模块路径。直接使用 my_macro!()

## 2. 使用 use 导入, `#[macro_use]`
之前的 `#[macro_export]` 是为了跨 crate 导出设计的，如果想在模块内导入，通常使用模块内的宏。
当在模块中定义的宏不使用 `#[macro_export]`, 但希望在其他模块中使用时，你可以用 `#[macro_use]` 来导入该模块。这会自动在整个 crate中引入该模块中所有的宏，在当前的 crate 中，使用更简单的方式来调用这些宏。

对应示例: macro3_1.rs, macro3_2.rs