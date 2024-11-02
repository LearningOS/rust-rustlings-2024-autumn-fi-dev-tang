/*
重新实现 42. modules2
重新理解所有关于 pub 变量和 use 的使用
use 作为重命名变量，也需要使用 pub use XXX; 来公开重命名
*/
mod delicious_snacks{
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    pub mod fruits{
        pub const PEAR: &'static str = "PEAR";
        pub const APPLE: &'static str = "APPLE";
    }

    pub mod veggies{
        pub const CUCUMBER: &'static str = "CUCUMBER";
        pub const CARROT: &'static str = "CARROT";
    }
}

fn main(){
    println!("fruit {} and vegetable {}", 
    delicious_snacks::fruit, delicious_snacks::veggie);
}