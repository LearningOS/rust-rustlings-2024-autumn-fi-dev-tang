/*
设置 8 个线程，每个线程轮流从共享数组中按偏移量获取对应元素的加和
用到 rust 标准库里处理多线程共享所有权的 std::sync::Arc;
建立共享引用对象, reference counter , 确保线程安全
最后需要父线程等待所有的子线程完成
*/
#![forbid(unused_imports)]
use std::sync::Arc;
use std::thread;

fn main(){
    let numbers :Vec<_> = (0..100u32).collect();  
    /*
    这里老是出现报错: 说 numbers 必须要注明类型，是因为 collect() 方法属于泛型
    收集成多种集合类型，例如 1. Vec<T> 2. HashSet<T> 3. HashMap<K,V>
    */
    let shared_numbers = Arc::new(numbers);
    let mut join_thread = Vec::new();

    for offset in 0..8{
        let child_numbers = Arc::clone(&shared_numbers);
        join_thread.push(
            thread::spawn(
                move || {
                    let sum: u32 = child_numbers.iter().filter(|v| (**v) % 8 == offset).sum();
                    println!("Thread {} sum: {}", offset, sum);
                }
            )
        );
    }

    for handle in join_thread.into_iter(){
        handle.join().unwrap();
    }
}