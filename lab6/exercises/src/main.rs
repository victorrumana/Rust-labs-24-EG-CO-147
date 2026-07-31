mod arc;
mod channels;
mod file;
mod async_await;
mod stretch_goal6;

use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("========================================");
    println!("     LAB 6: CONCURRENCY & FILE I/O      ");
    println!("========================================\n");

    // --- 6.2 EXERCISE A: SPAWNING THREADS ---
    println!("--- Exercise A: Spawning Threads ---");
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("[thread] count = {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..=3 {
        println!("[main] count = {}", i);
        thread::sleep(Duration::from_millis(80));
    }

    handle.join().expect("Thread panicked");
    println!("All done!");

    // TODO 1: Spawn 4 threads, each computing sum of a quarter of (1..=1000)
    let ranges = vec![1..=250, 251..=500, 501..=750, 751..=1000];
    let mut handles = vec![];

    for r in ranges {
        let h = thread::spawn(move || -> u64 { r.sum() });
        handles.push(h);
    }

    let mut total_sum: u64 = 0;
    for h in handles {
        total_sum += h.join().expect("Thread failed");
    }
    println!("Total sum of 1..=1000 across 4 threads (TODO 1): {}\n", total_sum);

    // --- CALLING OTHER EXERCISES ---
    arc::run();
    println!();

    channels::run();
    println!();

    let _ = file::run();
    println!();

    async_await::run().await;

    //------ stretch goal-------
    println!("\n Running exercise 6");
    stretch_goal6::run();
}