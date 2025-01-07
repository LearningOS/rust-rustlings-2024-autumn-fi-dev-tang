/*
一个关于 multi producer single consumer 的实现
调用了 rust 的标准库 std::sync::mpsc 
这里设置了多个 Sender 和一个 Receiver 
发送数组消息并查看对应的结果, 使用 mpsc::channel() 创建一个异步消息通道，返回 Sender 和 Receiver

目前感觉这个设计开始合理的，首先 Queue 分成了两部分
其次 thread::sleep, 导致每次调用必然会出现先从 first_half 取数，再从 second_half 取数

关于 for receiver in rx 的使用
for 循环把 rx 转换成一个迭代器，每次迭代的过程中调用 recv() 方法获取一个消息
*/
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

struct Queue{
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self{
        Queue{
            length: 10,
            first_half: vec![1,2,3,4,5],
            second_half: vec![6,7,8,9,10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> (){
    // 创建多个线程时，Queue 需要创建对应的引用计数对象 Arc
    // tx 本身自带的 clone, 也需要对每个线程赋所有权
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);  // 供本次调用的第一个线程使用
    let qc2 = Arc::clone(&qc);  // 供本次调用的第二个线程使用

    let tx1 = tx.clone();
    let tx2 = tx.clone();

    thread::spawn(move || {
        for value in &qc1.first_half {
            tx1.send(*value).unwrap();
            println!("Send: {}", value);
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for value in &qc2.second_half {
            tx2.send(*value).unwrap();
            println!("Send: {}", value);
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}

fn main(){
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let length = queue.length;

    send_tx(queue, tx);
    let mut total_length : u32 = 0;
    for receiver in rx {
        println!("Receive: {}", receiver);
        total_length += 1;
    }

    assert_eq!(total_length,length);
}