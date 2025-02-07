// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if let Some(item) = my_option{
        // 直接去掉 my_option.unwrap(), 因为 if let Some(item) = my_option 已经安全解包
        // 这里的 Option 包裹的是一个单元类型 ()
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    /*
    问题在于 my_arr.resize(0, 5)
    第一个参数是修改数组的长度为 5, 直接返回一个单元类型() ,表示无信息量，所以这里的绑定没有意义，直接删掉 let binding
     */
    println!("This Vec is empty, see? {:?}", ());

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    /*
    这里重新回顾一下所有权的问题
    std::mem::swap(&mut value_a, &mut value_b);
    接收的是两个变量的可变引用，执行之后 value_a 和 value_b 的值会互换，
    但是它们的所有权仍然分别属于 value_a 和 value_b
    不会改变变量的所有权，只是改变它们所指向的值。
    在不转移所有权的情况下交换两个变量的值
     */
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}