/*
写在最前:
之前意识到 Rust 中的 vec! 使用宏来实现任意长度，任意类型的动态数组
相较于普通的函数而言，使用宏实现能够 
【无限制增加元素】
rust里面限制了参数的个数，而宏编程可以通过不断进行模式匹配，匹配零个到任意长度个的数组进行实现

声明式宏编程
类似 rust 中的 match 模式匹配，使用 target 来匹配，执行对应条件的源代码，
区别在于 宏编程使用代码进行匹配，用匹配成功的条件中的代码来替代源代码
*/
#[macro_export]
// macro_export 说明在包外也能够使用这个声明宏
macro_rules! simu_vec {
    // 对输入的模式进行匹配，通常的 vec![1,2,3] 需要匹配零个或多个逗号表达式
    // 最外层的 $(), * 表示零个或多个逗号表达式
    // 内层中使用 $x: expr 表示一个 rust 表达式
    ( $( $x:expr ), *   ) => {
       {
        let mut temp_vec = Vec::new(); 
        // $()* 匹配多个逗号表达式
        $(
            temp_vec.push($x);
        )*
        temp_vec
       }
    }
}

fn main(){
    let v1 = simu_vec![1, 2, 3];
    let v2 = simu_vec!["After a while", "I understand", "macro programming"];
    println!("{:?}", v1);
    println!("{:?}", v2);
}