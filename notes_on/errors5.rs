/**
 * 一个比较 standard 的错误处理流程
 * 仿照这种写法
 * 
 * [psNote]: 这里详细记录一下关于 Box<dyn ...> 的使用，impl fmt::Display, 是出于打印的目的，可以理解
 * impl error::Error 自定义一个 Error 的类型
 * 
 * Rust 中，Box<dyn error::Error> 表示一个指向实现了 `std::error::Error` 特征对象的指针
 * 1. Box
 * Box 是一个智能指针，用于在堆上分配内存。使用 `Box` 允许动态使用内存，对于大小未知或运行时决定的类型尤其有用。
 * 
 * 2. dyn
 * dyn 是 "dynamic(动态)"的缩写，表示动态分发。将 dyn 用于特征(trait), 如 dyn error::Error,
 * 表示编译器将在运行时查找方法调用的具体实现，而不是在编译时决定哪个实现被调用。
 * 这种方法被称为动态调度或动态派发(dynamic dispatch), 通常用于处理不同的类型实现相同的特征的情况。
 * 
 * 3. std::error::Error
 * std::error::Error 描述实现两个基本特征 Debug 和 Display, 允许提供错误的原因（实现可选的 `source` 方法）。
 * 通过实现这个特征，一个类型就可以被视为一种错误。
 * 
 * 4. 使用 Box<dyn error::Error>
 * 表示这个函数可能返回一个实现了 `std::error::Error` 特征的错误值。
 * 4.1. 灵活性
 * 处理 ParseIntError, CreationError 以及其他实现了 error::Error 特征的错误类型
 * 
 * 4.2. 一致性
 * 特征对象可以用来实现多态性，可以用统一的方式处理不同的错误类型
 * 
 * 4.3.减少复杂性
 * 程序包含多种可能的错误类型，不需要为每一种错误类型都定义一个单独的错误枚举
 * [目前认为这个 Box<dyn error::Error> 的使用非常精妙]
 */
use std::error;
use std::fmt;
use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError{
    Negative,
    Zero,
}

impl PositiveNonzeroInteger{
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError>{
        match value{
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

impl fmt::Display for CreationError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let description = match *self{
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };

        f.write_str(description)
    }
}

impl error::Error for CreationError {}

fn main() -> Result<(), Box<dyn error::Error>>{
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse().unwrap();
    println!("output = {:?}", PositiveNonzeroInteger::new(x));
    Ok(())
}