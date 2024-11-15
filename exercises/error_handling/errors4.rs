// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

/**
 * 一个更 rust 的写法，rigid 的要求就是尽量不要使用多个 if else 嵌套
 * match value {
 *  v if v > 0 => { PositiveNonzeroInteger(value as u64)}
 *  0 => {Err(CreationErr::Zero)}
 *  _ => {Err(CreationErr::Negative)}
 * }
 * 
 * 感觉这里第一条的 v if v > 0, 称为条件守卫，跟 Haskell 里面的 guard | 非常相像
 */

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        if value > 0 {
            Ok(PositiveNonzeroInteger(value as u64))
        }else{
            if value == 0 {
                Err(CreationError::Zero)
            }
            else{
                Err(CreationError::Negative)
            }
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}