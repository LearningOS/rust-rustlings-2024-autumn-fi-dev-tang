/*
这里是对之前的 Arc 的补充，
第一种情况是多个线程共享一个变量，但是只读，不修改
第二种情况是多个线程分别获取变量的所有权，同时对变量进行写操作，涉及到加锁
关于 std::sync::Mutex;
这里上锁之后，rust标准库的设计是，离开作用域之后释放掉这个锁。
[同时对于 thread::spawn(move || {})]; move 的common practice 是获取外部变量的所有权，
即使创建该线程的父线程结束之后，子线程还可以拥有对变量的所有权。
【psNote】: std::sync::Arc 和 std::sync::Mutex 的配合使用
*/
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

struct Job{
    jobs_completed: u32,
}

fn main(){
    // 1. 创建带有锁的线程共享变量, 引用计数的初始值为 1
    let shared_variable = Arc::new(Mutex::new(Job{jobs_completed: 0}));

    // 2. 收集 10 个线程的线程句柄
    let mut thread_handles = vec![];

    // 3. 依次创建10个线程，设置线程共享的引用计数，每次写操作之前获取锁
    /*
    最终的选择是把 child_shared_variable 放在 thread::spawn() 之外 
    目前我认为的原因是，其实闭包都可以捕获外部变量，所以本来放在内部或者外部区别不大，
    这里由于创建的时候 shared_variable 没有实现Copy trait，在被第一个子线程move 之后，其它线程就无法获得所有权
    */
    for i in 0..10{
        // pos 1.
        let child_shared_variable = Arc::clone(&shared_variable);
        thread_handles.push(
            thread::spawn( move || {
                // 这里考虑一下 child_shared_variable  放的位置, 或许 child_shared_variable 可以放在 thread::spawn() 之外 ?

                // pos 2.
                // 子线程获取这个引用计数
                let mut job = child_shared_variable.lock().unwrap(); // 子线程加锁
                job.jobs_completed += 1;
                println!("thread {} has job.completed = {}", i, job.jobs_completed);
            })
        );
    }

    // 4. 等待线程结束
    for handle in thread_handles {
        handle.join().unwrap();
    }

    // 5. 打印最终的引用计数
    println!("jobs_completed: {}", shared_variable.lock().unwrap().jobs_completed);
}