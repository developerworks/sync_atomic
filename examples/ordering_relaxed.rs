use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    // 创建一个原子计数器
    let counter = AtomicUsize::new(0);
    // 创建两个线程, 分别对计数器进行递增操作
    let thread1 = thread::spawn(move || {
        for _ in 0..1_000_000 {
            counter.fetch_add(1, Ordering::Relaxed);
        }
    });
    let thread2 = thread::spawn(move || {
        for _ in 0..1_000_000 {
            counter.fetch_add(1, Ordering::Relaxed);
        }
    });
    // 等待两个线程结束
    thread1.join().unwrap();
    thread2.join().unwrap();
    // 打印计数器的值
    println!("Counter: {}", counter.load(Ordering::Relaxed));
}
