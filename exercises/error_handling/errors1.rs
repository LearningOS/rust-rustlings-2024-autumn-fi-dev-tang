// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Result` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.

/**
 * [psNote]:
 * !!! 这个一开始自己没想到，认为还是对 String 类型等的使用范围理解不够灵活，没有想到 Result<T,E> 里面的 E 也可以使用一般的类型，而不是 Error 类型
 * 这里纯属是自己想复杂了，一开始想做Error处理，编译器报了很多的 Error 处理包选项，
 * 例如 use std::fmt::Error
 * 但是 assert 语句中使用的是 &str 使用 .into() 转换成 String 类型
 * 直接判断，assert 语句里就是 String 类型
 * Ok 和 Err 里面的 E 直接使用 String 类型
 * 其余类型可能会写成 Result<String, dyn Error>，或者 into() trait not implemented 的错误
 */
pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}