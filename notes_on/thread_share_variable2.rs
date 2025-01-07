/*
这里首先补一些关于 std::sync::Arc 的内容，来源是官方的 docs: https://rustwiki.org/zh-CN/std/sync/struct.Arc.html

1. Arc::new()
创建一个引用计数的 Arc 对象，这个对象会指向同一个实例(指针指向的地址一致)
2. Arc::clone()
每次 Clone() 操作之后，创建一个新的 Arc 对象，Arc 对象包含指向同一个位置的指针，同时增加对原来的 Arc 对象的引用计数
(原子操作保证线程安全性)

创建 10 个线程，同时设置对共享内容的访问计数, 访问时上锁
*/
use std::sync::{Arc, Mutex};
use std::thread;

struct Jobinfo{
    job_completed: u32,
}

fn main(){
    // 1. 主线程中设置供 10 个线程共享的引用计数变量
    let shared_variable = Arc::new(Mutex::new(Jobinfo{job_completed: 0}));

    // 2. 创建 10 个线程的向量
    let mut thread_handles = vec![];

    // 3. 记录每个线程的私有访问情况
    for i in 0..10 {
        /*
        shared_variable 只能被 move 一次，实际上任何变量都只能被 move 一次
        这里在主线程中克隆出 10 个 Arc 实例，在主线程中访问 shared_variable 是合法的
        */
        let child_shared_variable = Arc::clone(&shared_variable);
        thread_handles.push(
            thread::spawn(
                move || {
                    thread::sleep(std::time::Duration::from_secs(1));
                    let mut job_acquire_lock = child_shared_variable.lock().unwrap();
                    job_acquire_lock.job_completed += 1;
                    println!("thread {} 's completed_job {}", i, job_acquire_lock.job_completed);
                }                
            )
        );
    }

    // 4. 等待线程结束
    for handle in thread_handles {
        handle.join().unwrap();
    }

    println!("All threads are finished, with completed_job {}", shared_variable.lock().unwrap().job_completed);
}