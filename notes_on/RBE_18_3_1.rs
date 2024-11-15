/* Error handling 
其实按照实例程序的 map 能懂，比如 Enum Food -> struct Peeled(Food) -> Struct Chopped(Food) -> Struct Cooked(Food)
都很直接，and_then 没有完全理解
*/
use std::error::ParseIntError;

fn multiply(first_string_number: &str, second_string_number: &str) -> Result< i32, ParseIntError>{
    match first_string_number.parse::<i32>() {
        Ok(first_number) => {
            match second_string_number.parse::<i32>(){
                Ok(second_number) => {
                    first_number * second_number
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e)
    }
}

fn main(){
    let first_string_number = "123";
    let second_string_number = "456";

    match multiply(first_number, second_number) {
        Ok(result) => println!("{}", result),
        Err(e) => Err(e),
    }
}