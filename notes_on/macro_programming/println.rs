/*
rust 里面的 println! 是宏，宏后面可以跟 (), [], {}
*/
fn main(){
    println!("aaa");
    println!["aaa"];
    println!{"aaa"};
}