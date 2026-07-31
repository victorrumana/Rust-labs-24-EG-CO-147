Lab Session 6: Concurrency & File I/O

This directory contains the solutions for **Lab Session 6**, focusing on multithreading, shared state synchronization, message passing, file operations, and async programming using Tokio.

## 📝 Exercises & Implementation Details

* **Exercise A — Spawning Threads:** 
  Demonstrates OS thread creation using `std::thread::spawn` and distributing summation workloads across 4 parallel threads using `move` closures.
* **Exercise B — Shared State with `Arc<Mutex<T>>`:** 
  Demonstrates safe thread synchronization. Implements an optimized local-accumulation strategy to minimize lock contention and improve concurrency performance.
* **Exercise C — Channels (Message Passing):** 
  Uses `std::sync::mpsc` channels to distribute work across worker threads and transmit structured `WorkResult` message variants.
* **Exercise D — File I/O & Directory Traversal:** 
  Covers file creation, line buffering (`BufReader`), string pattern searching, and recursive file tree directory traversal using `std::fs`.
* **Exercise E — Async/Await with Tokio:** 
  Demonstrates concurrent network/latency fetching using `tokio::spawn` and awaiting multiple futures simultaneously using `tokio::join!`.
