use std::{sync::Arc, thread, time::Instant};

use crossbeam::atomic::AtomicCell;

fn main() {
    let start = Instant::now();
    // 创建一个原子计数器
    let counter = Arc::new(AtomicCell::new(0usize));

    // 创建两个线程, 分别对计数器进行递增操作
    let counter1 = counter.clone();
    let thread1 = thread::spawn(move || {
        for _ in 0..10_000_000 {
            counter1.fetch_add(1);
        }
    });
    let counter2 = counter.clone();
    let thread2 = thread::spawn(move || {
        for _ in 0..10_000_000 {
            counter2.fetch_add(1);
        }
    });

    // 等待两个线程结束
    thread1.join().unwrap();
    thread2.join().unwrap();

    // 打印计数器的值
    println!("Counter: {}", counter.load());
    println!("Time: {} ms", start.elapsed().as_millis());
}
