use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
pub enum WorkResult {
    Sum(u64),
    Error(String),
}

pub fn worker(id: usize, data: Vec<u64>, tx: mpsc::Sender<WorkResult>) {
    let sum = data.iter().sum();

    // TODO 3: Add an Error variant if a chunk's sum exceeds 30,000
    if sum > 30_000 {
        let _ = tx.send(WorkResult::Error(format!(
            "Worker {} sum exceeded limit: {}",
            id, sum
        )));
    } else {
        println!("Worker {} computed sum = {}", id, sum);
        let _ = tx.send(WorkResult::Sum(sum));
    }
}

pub fn run() {
    println!("--- Exercise C: Channels ---");
    let (tx, rx) = mpsc::channel();
    let dataset: Vec<Vec<u64>> = (0..4)
        .map(|i| ((i * 250 + 1)..=(i + 1) * 250).collect())
        .collect();

    for (id, chunk) in dataset.into_iter().enumerate() {
        let tx_clone = tx.clone();
        thread::spawn(move || worker(id, chunk, tx_clone));
    }

    drop(tx);

    let mut grand_total: u64 = 0;
    for res in rx {
        match res {
            WorkResult::Sum(s) => grand_total += s,
            WorkResult::Error(err) => println!("Received Error: {}", err),
        }
    }

    println!("Grand total: {} (expected 125250)", grand_total);
}