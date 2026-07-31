// src/functions.rs

fn add(a: i32, b: i32) -> i32 {
    a + b    // No semicolon = expression = return value
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn run() {
    println!("{}", add(3, 7));
    println!("{}", greet("Rustacean"));

    // TODO 6: Write a recursive function `factorial(n: u64) -> u64`
    //          and print factorial(10).
    let result = factorial(10);
    println!("factorial(10) = {}", result);
}

// TODO 6 — implement factorial below
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1 // Base case: factorial of 0 or 1 is 1
    } else {
        n * factorial(n - 1) // Recursive case
    }
}