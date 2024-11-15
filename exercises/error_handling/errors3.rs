// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

/**
 * 主要考察 rust 里的 unwrap() 和 unwrap_err() 使用
 * 1. unwrap() 方法用于从一个 `Option` 或 `Result` 中提取信息，
 * Some(value) 和 Ok(value) 的情况下返回 value, None 和 Err(error) 的情况下导致程序崩溃，打印错误信息
 * 
 * 2. unwrap_err() 从程序中提取错误值 E, 返回 error
 * 如果 unwrap_err() 遇到 Ok(value) 会 panic 导致程序崩溃
 */
use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input).unwrap();

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    match item_quantity.parse::<i32>(){
        Ok(qty) => Ok(qty * cost_per_item + processing_fee),
        Err(e) => Err(e),
    }
}