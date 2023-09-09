use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    let start = std::time::Instant::now();
    let counter = Arc::new(AtomicUsize::new(0));

    let counter1 = Arc::clone(&counter);
    let thread1 = thread::spawn(move || {
        for _ in 0..10 {
            counter1.fetch_add(1, Ordering::AcqRel);
            println!("thread1 add one");
        }
    });
    let counter2 = Arc::clone(&counter);
    let thread2 = thread::spawn(move || {
        for _ in 0..10 {
            counter2.fetch_sub(1, Ordering::AcqRel);
            println!("thread2 substract one");
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Counter: {}", counter.load(Ordering::Acquire));
    println!("Time: {}ms", start.elapsed().as_millis());
}
