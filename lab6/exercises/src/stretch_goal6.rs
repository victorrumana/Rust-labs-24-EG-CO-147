use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn run() {
    println!("=== LAB 6 STRETCH GOAL ===");

    let urls = vec![
        "https://www.rust-lang.org",
        "https://crates.io",
        "https://doc.rust-lang.org",
        "https://github.com",
        "https://example.com",
        "https://httpbin.org/get",
        "https://www.wikipedia.org",
        "https://www.python.org",
        "https://www.gnu.org",
        "https://www.w3.org",
    ];

    let semaphore = Arc::new(Semaphore::new(3)); // Max 3 requests at once
    let client = reqwest::Client::new();
    let mut handles = vec![];

    for url in urls {
        let sem = Arc::clone(&semaphore);
        let client_clone = client.clone();
        let url = url.to_string();

        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap(); // Acquire 1 permit slot
            
            match client_clone.get(&url).send().await {
                Ok(resp) => match resp.text().await {
                    Ok(text) => {
                        let word_count = text.split_whitespace().count();
                        (url, word_count, "Success".to_string())
                    }
                    Err(_) => (url, 0, "Read Error".to_string()),
                },
                Err(_) => (url, 0, "Request Error".to_string()),
            }
        }));
    }

    let mut results = vec![];
    for handle in handles {
        if let Ok(res) = handle.await {
            results.push(res);
        }
    }

    // Write results to summary CSV file
    let mut file = File::create("scraper_summary.csv").await.unwrap();
    file.write_all(b"URL,WordCount,Status\n").await.unwrap();

    for (url, count, status) in &results {
        let line = format!("{},{},{}\n", url, count, status);
        file.write_all(line.as_bytes()).await.unwrap();
        println!("Processed: {} | Words: {} | Status: {}", url, count, status);
    }

    println!("Summary written to scraper_summary.csv");
}