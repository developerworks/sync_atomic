// #![feature(mutex_unlock)]

use std::sync::atomic::{AtomicBool, Ordering, AtomicUsize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    // 创建一个共享数据
    let data = Arc::new(Mutex::new(AtomicUsize::new(0)));
    // 生产者完成标记
    let producer_done = Arc::new(AtomicBool::new(false));
    // 共享数据引用副本
    let data_clone = data.clone();
    // 生产者完成标记引用副本
    let producer_done_clone = producer_done.clone();
    // 生产者线程
    let producer = thread::spawn(move || {
        // record start time
        let start = Instant::now();
        // 获得锁
        let data = data_clone.lock().unwrap();
        for _i in 0..10 {
            thread::sleep(Duration::from_nanos(1));
            // data.push(i);
            data.fetch_add(1, Ordering::SeqCst);
        }
        // 修改完成标记
        producer_done_clone.store(true, Ordering::Release);
        // output elapsed time
        println!("Elapsed time [producer]: {:?}", start.elapsed());
    });
    // 消费者线程
    let consumer = thread::spawn(move || {
        let start = Instant::now();
        while !producer_done.load(Ordering::Acquire) {
            // Wait for the producer to finish.
            thread::sleep(Duration::from_nanos(2));
            println!("Wait for the producer to finish.");
        }
        // 获取
        let data = data.lock().unwrap();
        println!("Data: {:?}", *data);
        // data.unlock()
        println!("Elapsed time [consumer]: {:?}", start.elapsed());
    });
    // 主线程等待子线完成
    producer.join().unwrap();
    consumer.join().unwrap();
}
