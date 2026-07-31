// src/ownership.rs

pub fn run() {
    // — Move semantics 

    let s1 = String::from("hello");
    let s2 = s1; 
    // s1 is MOVED into s2
    println!("s2 = {}", s2);

    // — Clone (deep copy) 

    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4); // Both valid

    // — Borrowing (immutable reference) 

    let s5 = String::from("Rust is great");
    let length = calculate_length(&s5);    // pass reference
    println!("'{}' has {} characters", s5, length);

    // // TODO 5: Write a function `first_word(&str) -> &str`
    // // that returns a slice of the first word.
    // // Call it here and print the result.
    let word = first_word(&s5);
    println!("The first word of s5 is: {}", word);
}

fn calculate_length(s: &String) -> usize {
    s.len() // s is borrowed; not dropped at end of scope
}

// TODO 5 — implement first_word below

fn first_word(s: &str) -> &str {
    // Use .find(' ') to locate the position of the first space character
    match s.find(' ') {
        Some(pos) => &s[..pos], // If a space is found, return the slice up to that index
        None => s,              // If no space is found, the whole string is one word
    }
}