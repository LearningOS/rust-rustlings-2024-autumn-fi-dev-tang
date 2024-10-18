pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// 餐厅前厅，用于吃饭
/*
修改版：
告诉Rust 从另一个模块 front_of_house 同名的文件中加载该模块的内容
使用绝对路径的方式来引用 hosting 模块: crate::front_of_house::hosting;
*/
mod front_of_house;

pub use crate::front_of_house::hosting;

// 使用路径来引用模块，用相对路径或绝对路径来引用
// 实现小功能
// 一个定义在包根中的函数
pub fn eat_at_restaurant(){
    // 绝对路径
    //crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    //front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

/*
[error]:
运行报错，
front_of_house::hosting::add_to_waitlist(); 
错误命令: private module, hosting 模块是私有化的。无法在包根进行访问。
front_of_house 模块可以访问的原因: 它和 eat_at_restaurant 同属于一个包根作用域内，同一个模块内的代码不存在私有化问题。
*/
fn serve_order(){
    self::back_of_house::cook_order()
}

// 厨房模块
mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        /*
        确定未来层级模式不会改变， super::serve_order 的方式会更稳定
        */
        super::serve_order();
    }

    pub fn cook_order(){}
}