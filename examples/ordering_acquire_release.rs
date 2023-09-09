// #![feature(mutex_unlock)]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
// use std::time::Duration;

fn main() {
    // 创建一个共享数据
    let data = Arc::new(Mutex::new(vec![]));
    // 生产者完成标记
    let producer_done = Arc::new(AtomicBool::new(false));
    // 共享数据引用副本
    let data_clone = data.clone();
    // 生产者完成标记引用副本
    let producer_done_clone = producer_done.clone();
    // 生产者线程
    let producer = thread::spawn(move || {
        // 获得锁
        let mut data = data_clone.lock().unwrap();
        for i in 0..10 {
            // thread::sleep(Duration::from_nanos(1));
            data.push(i);
        }
        // 修改完成标记
        producer_done_clone.store(true, Ordering::Release);
    });
    // 消费者线程
    let consumer = thread::spawn(move || {
        while !producer_done.load(Ordering::Acquire) {
            // Wait for the producer to finish.
            println!("Wait for the producer to finish.");
        }
        // 获取
        let data = data.lock().unwrap();
        println!("Data: {:?}", *data);
        // data.unlock()
    });
    // 主线程等待子线完成
    producer.join().unwrap();
    consumer.join().unwrap();
}
