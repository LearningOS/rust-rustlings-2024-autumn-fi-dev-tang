/**
 * 自定义 std::error::Error 的实现
 * 使用 Box<dyn error::Error>
 * 1. 能够支持分配在栈上的动态内存，运行时分配而不是编译时分配
 * 2. dyn 返回值类型 Result<(), Box<dyn error::Error>> 运行时动态查找当前实现 std::error::Error 的对象，支持多态
 */
use std::fmt;
use std::error;

#[derive(Debug, PartialEq)]
struct PositiveNonzeroInteger(u64);

#[derive(Debug, PartialEq)]
enum CreationError{
    Negative,
    Zero,
}

impl PositiveNonzeroInteger{
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError>{
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match *self {
            CreationError::Negative => "this is an negative number",
            CreationError::Zero => "this is an zero number",
        };
        f.write_str(output)
    }
}

impl error::Error for CreationError{}

fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x = pretend_user_input.parse().unwrap();
    println!("The value is {:?}", PositiveNonzeroInteger::new(x));
    Ok(())
}