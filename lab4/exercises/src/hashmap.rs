use std::collections::HashMap;

pub fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
        let clean: String = word
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_lowercase().next().unwrap())
            .collect();

        if !clean.is_empty() {
            *freq.entry(clean).or_insert(0) += 1;
        }
    }
    freq
}

// TODO 3: Return top n words by frequency, sorted descending
pub fn top_n(freq: &HashMap<String, usize>, n: usize) -> Vec<(&String, &usize)> {
    let mut items: Vec<(&String, &usize)> = freq.iter().collect();
    items.sort_by(|a, b| b.1.cmp(a.1)); // Sort descending by count
    items.into_iter().take(n).collect()
}

pub fn run() {
    let text = "the quick brown fox jumps over the lazy dog the fox was very quick and the dog was lazy";
    let freq = word_frequency(text);
    println!("Word frequencies: {:?}", freq);

    println!("\nTop 5 words:");
    for (word, count) in top_n(&freq, 5) {
        println!("{}: {}", word, count);
    }
}