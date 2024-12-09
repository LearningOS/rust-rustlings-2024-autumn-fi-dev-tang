/*
这一部分是对 &str 类型的迭代器使用实例:
分别构造三个程序，完成下列功能:
1. 把开头字母大写
2. 对 &[&str] 中每个单词都开头字母大写
3. 把 &[&str] 中的每个单词进行合并
*/
// 1. 对 &str 进行迭代
fn capitalize_first(capital: &str) -> String{
    let mut c = capital.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            let mut capitalize_char = first.to_uppercase().collect::<String>();
            capitalize_char.push_str(c.as_str());
            capitalize_char
        },
    }
}

// 2. 对 &[&str] 进行迭代
fn capitalize_words_vector(words: &[&str]) -> Vec<String>{
    let mut vec_string : Vec<String> = Vec::new();
    for word in words.iter() {
        vec_string.push(capitalize_first(word))
    }
    vec_string
}

// 3. 合并 &[&str] 成 String
fn capitalize_words_string(words: &[&str]) -> String {
    let mut vec_string : String = String::new();
    for word in words.iter() {
        vec_string.push_str(capitalize_first(word).as_str())
    }
    vec_string
}


fn main(){
    let  little_capital :&str = "little capital";
    println!("To {}",capitalize_first(little_capital));

    let little_capital_vec = &["hello", "little", "capital"];
    for word in capitalize_words_vector(little_capital_vec).iter() {
        print!("{} ", word);
    }
    println!("");

    let bigger_capital_vec = &["hello ", "little ", "capital "];
    println!("{} ", capitalize_words_string(bigger_capital_vec));
}