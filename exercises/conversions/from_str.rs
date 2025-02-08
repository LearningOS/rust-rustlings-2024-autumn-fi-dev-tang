// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}


// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an
//    error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error
//    should be returned
// If everything goes well, then return a Result of a Person object
//
// As an aside: `Box<dyn Error>` implements `From<&'_ str>`. This means that if
// you want to return a string error message, you can do so via just using
// return `Err("my error message".into())`.

impl FromStr for Person {
    type Err = ParsePersonError;
    /*
    回顾之前的一个知识点，type 的作用是创建类型别名
    就是为了表达的简略和方便，使用 Err 来代替 ParsePersonError
    下面的 Err 就当成是 ParsePersonError 来使用
     */
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // 0. 直接判断 Empty 类型
        if s.is_empty() {
            return Err(Self::Err::Empty);
        }
        // 1. 还是按照之前的逗号模式来
        let comma_count = s.matches(',').count();
        if comma_count != 1 {
            return Err(Self::Err::BadLen);
        }
        // 2. 接下来逗号模式匹配结束，都是只有一个逗号的情况，字符串 name 和 age 部分的匹配
        let parts: Vec<&str> = s.split(',').collect();
        let (name_part, age_part) = (parts[0], parts[1]);
        
        match name_part {
            "" => {
                // name_part 是空字符串，测试后面第二部分的情况 
                Err(Self::Err::NoName)
            }
            _ => {
                match age_part.parse::<usize>() {
                   Ok(age) => {
                        Ok(Person{
                            name: name_part.to_string(),
                            age: age,
                        })
                   },
                   Err(e) => {
                        Err(Self::Err::ParseInt(e))
                   }
                }
            }
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
