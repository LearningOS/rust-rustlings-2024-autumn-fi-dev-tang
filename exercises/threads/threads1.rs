// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];

    /*
    创建了一个空的 Vec<JoinHandle<u128>> 向量，用于存储每个线程的句柄。
    handles 用来保存所有线程的句柄，以便稍后等待它们完成。
    JoinHandle<T> 是一个智能指针，允许主线程等待子线程完成并获取其返回值。
    在这个例子里，T 是 u128, 每个线程将返回一个 u128 类型的值，表示线程的执行时间。
    */

    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];        // 存储每个线程的执行时间
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        results.push(handle.join().unwrap());  
        // handle.join() 阻塞主线程，直到相应的子线程完成。
        /*
        handle.join() 阻塞主线程，直到相应的子线程完成。join() 返回一个 Result<T,E>, 
        其中 T 是线程的返回值，在这里是 u128, E 是可能的错误类型，使用 unwrap() 处理 Result。
        */
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}