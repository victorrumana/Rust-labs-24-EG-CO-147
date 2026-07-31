use tokio::time::{sleep, Duration, Instant};

pub async fn fetch_data(id: u32) -> String {
    sleep(Duration::from_millis(100)).await;
    format!("Data from source {}", id)
}

pub async fn run() {
    println!("--- Exercise E: Async/Await with Tokio ---");
    
    println!("// Sequential - slow");
    let t0 = Instant::now();
    for id in 1..=4 {
        let data = fetch_data(id).await;
        println!("{}", data);
    }
    println!("Sequential time: {:?}", t0.elapsed());

    println!("\n// Concurrent - fast");
    let t1 = Instant::now();
    let mut handles = vec![];
    for id in 1..=4 {
        handles.push(tokio::spawn(fetch_data(id)));
    }

    for h in handles {
        let data = h.await.unwrap();
        println!("{}", data);
    }
    println!("Concurrent time: {:?}", t1.elapsed());

    // TODO 5: Use tokio::join! to await all 4 futures simultaneously
    println!("\n// tokio::join! (TODO 5)");
    let t2 = Instant::now();
    let (res1, res2, res3, res4) = tokio::join!(
        fetch_data(1),
        fetch_data(2),
        fetch_data(3),
        fetch_data(4)
    );

    println!("{}", res1);
    println!("{}", res2);
    println!("{}", res3);
    println!("{}", res4);
    println!("tokio::join! time: {:?}", t2.elapsed());
}