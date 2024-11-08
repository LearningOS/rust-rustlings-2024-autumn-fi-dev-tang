/*
模拟这段 while let Some() = Option<T> 的写法
*/
fn layered_option(){
    let range = 10;
    let mut optional_integers: Vec<Option<i8>> = vec![None];

    for i in 1..(range + 1){
        optional_integers.push(Some(i));
    }
    let mut cursor = range;

    while let Some(integer) = optional_integers.pop(){
        println!("integer = {:?}", integer);
    }
}

fn main(){
    layered_option();
}

/*
解释第 49 题中遇到的问题:
函数中使用的是 `while let` 表达式，这种匹配模式专门用来处理 `option` 或者其他模式匹配的情景。

while let Some(integer) = optional_integers.pop() 中

optional_integers.pop() 的类型是 `Option<Option<u8>>`, 
while let Some(integer) 中匹配的 Some() 内容是原先放入向量的内容，即 Option<u8>

rust-analyzer 里面对 pop() 的定义:
pub fn pop(&mut self) -> Option<T>{
    if self.len == 0 {
        None
    }else{
        unsafe{
            self.len -= 1;
            core::hint::assert_unchecked(self.len < self.capacity());
            Some(ptr::read(self.as_ptr().add(self.len()))
        }
    }
}

认为这条语句中关于 Pop() 的定义，对数组中的元素进行了二次包裹:
原来数组中的元素是 Option<T> 类型，现在使用
Some(ptr::read(self.as_ptr().add(self.len)))
将原来的 Option<u8> 继续包裹成 Some(Option<u8>),
换言之，pop() 出来的内容是 Some(Some(i)) 或者 None

while let Some(integer) == None 匹配不上，程序自动退出
*/