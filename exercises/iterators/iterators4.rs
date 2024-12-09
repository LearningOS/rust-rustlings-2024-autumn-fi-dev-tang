// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

/*
一个正确实现的例子:
(1..=num).fold(1, |acc, x| acc * x)
1. 创建一个包含从 1 到 num(包括 num)的范围迭代器
2. fold 是一个高阶函数，接收一个初始值(1) 和一个闭包。
闭包有两个参数: acc(累加器) 和 x (当前元素)。
每次迭代中，fold 将累加器与当前元素相乘，并将结果传递给下一次迭代。最终 fold 返回累加器的最终值，即阶乘的结果

*/
pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    // let mut result: u64 = 1;
    // for i in 2..=num {
    //     result *= i;
    // }
    // result
    // 上面我自己的实现没有满足要求
    (1..=num).fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
