# 关于 41. module2.rs
原来的程序可以修改成
```rs
mod delicious_snacks{
    // 1. 第一步: 公开子模块，在子模块上加上 pub
    // 2. 第二步: 修改 use 模块
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    pub mod fruits{
        pub const PEAR :&'static str = "Pear";
        pub const APPLE :&'static str = "Apple";
    }

    pub mod veggies{
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }

    // 第三步: 公开重命名的变量
}

fn main(){
    println!(
    "favorite snacks: {} and {}",
    delicious_snacks::fruit,
    delicious_snacks::veggie
    );
}

fn main(){
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
```