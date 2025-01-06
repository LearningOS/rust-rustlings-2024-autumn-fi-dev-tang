/*
这里主要想回顾创建多个 thread, 以及线程返回值的获取
*/
use std::thread;
use std::time::{Duration, Instant};

fn main(){
    // 1. 创建多个线程
    let mut thread_vec = vec![]; // 应该是存储线程的句柄，JoinHandler<T>, T 标志每个线程的返回值

    for i in 0..10{
        thread_vec.push(
            thread::spawn(
                move || {       
                    /*
                    thread::spawn 使用 move 关键字，将所有捕获的变量转移到主线程种，即使主线程结束，
                    新线程可以安全访问, 这里是 i 的 ownership
                    */
                    let start = Instant::now();
                    thread::sleep(Duration::from_secs(1));
                    println!("thread {} is done", i);
                    start.elapsed().as_millis()   // 线程返回值
                }
            )
        );
    }

    let mut result : Vec<u128> = vec![];
    for handle in thread_vec{
        result.push(handle.join().unwrap());
    }

    if result.len() != 10 {
        panic!("Error result length");
    }

    for (i, time) in result.into_iter().enumerate(){
        println!("thread {} is done in {}", i, time);
    }
}