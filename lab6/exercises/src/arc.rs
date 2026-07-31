use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

pub fn run() {
    println!("--- Exercise B: Shared State with Arc<Mutex<T>> ---");

    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = vec![];

    for _ in 0..8 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1_000 {
                let mut num = c.lock().unwrap();
                *num += 1;
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("Final counter: {}", *counter.lock().unwrap());

    // TODO 2: Refactor so each thread accumulates a local sum
    // and only locks the mutex once per thread at the end.
    let counter2 = Arc::new(Mutex::new(0u64));
    let mut handles2 = vec![];

    let start = Instant::now();
    for _ in 0..8 {
        let c = Arc::clone(&counter2);
        handles2.push(thread::spawn(move || {
            let mut local_sum = 0u64;
            for _ in 0..1_000_000 {
                local_sum += 1;
            }
            let mut num = c.lock().unwrap();
            *num += local_sum;
        }));
    }

    for h in handles2 {
        h.join().unwrap();
    }
    let duration = start.elapsed();

    println!("Optimized Final counter (TODO 2): {}", *counter2.lock().unwrap());
    println!("Time taken with local accumulation: {:?}", duration);
}