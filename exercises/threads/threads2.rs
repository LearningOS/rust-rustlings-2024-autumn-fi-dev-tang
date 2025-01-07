// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;

#[derive(Debug)]
struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 使用 std::sync::Mutex 和 std::sync::Arc 配合使用，对共享变量进行加锁
    // 原来的写法是: let status = Arc::new(JobStatus{jobs_completed: 0});
    let status = Arc::new(Mutex::new(JobStatus{jobs_completed: 0}));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            /*
            这里我认为对共享变量进行更新时，需要进行加锁操作,
            引入 rust 中的 use std::sync::Mutex; 包进行修改
            */
            let mut status_shared = status_shared.lock().unwrap();
            status_shared.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        println!("jobs completed {:?}", *status.lock().unwrap());
    }
}