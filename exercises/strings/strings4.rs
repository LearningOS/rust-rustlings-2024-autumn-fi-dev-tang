// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

/*
[psNote]:
I took a look at the Into trait, which is implemented after Sized trait
pub trait Into<T>: Sized { fn into(self) -> T }
which Sized is an inherent trait for every type, example followed like these:
1. str (false)  &str (true: because it is an reference.)
2. dyn ToString(false), must wrap it in an pointer, &dyn ToString, Box<dyn ToString> (Box<[T]> or &str or Box<dyn Trait>)
Box<[T]>, &str, Box<dyn Trait>, these types are Sized.

Into is used to transfer self to T
*/
fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string_slice("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}