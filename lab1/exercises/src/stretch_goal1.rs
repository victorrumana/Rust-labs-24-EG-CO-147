pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

pub fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();
    
    cleaned.chars().eq(cleaned.chars().rev())
}

pub fn run() {
    println!("=== LAB 1 STRETCH GOAL ===");
    let temp_f = celsius_to_fahrenheit(25.0);
    println!("25°C in Fahrenheit: {:.1}°F", temp_f);

    let word1 = "racecar";
    let word2 = "hello";
    println!("Is '{}' a palindrome? {}", word1, is_palindrome(word1));
    println!("Is '{}' a palindrome? {}", word2, is_palindrome(word2));
}