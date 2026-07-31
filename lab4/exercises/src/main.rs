mod hashmap;
mod closures_iterators;
mod custom_iterator;
mod stretch_goal4;

pub fn stats(data: &[f64]) -> (f64, f64, f64) {
    let sum: f64 = data.iter().sum();
    let mean = sum / data.len() as f64;
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    (mean, min, max)
}

// TODO 1: Compute median
pub fn median(scores: &[f64]) -> f64 {
    let len = scores.len();
    if len == 0 {
        return 0.0;
    }
    if len % 2 == 1 {
        scores[len / 2]
    } else {
        (scores[(len / 2) - 1] + scores[len / 2]) / 2.0
    }
}

// TODO 2: Compute variance and standard deviation
pub fn variance_and_std_dev(scores: &[f64]) -> (f64, f64) {
    let (mean, _, _) = stats(scores);
    let variance = scores.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / scores.len() as f64;
    let std_dev = variance.sqrt();
    (variance, std_dev)
}

fn main() {
    // --- Exercise A Output ---
    println!("--- Exercise A: Vec Operations ---");
    let mut scores: Vec<f64> = vec![85.0, 92.0, 78.5, 95.0, 60.0, 88.0];

    // Sorting
    scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("Sorted: {:?}", scores);

    // Iterator chain
    let high_scores: Vec<f64> = scores
        .iter()
        .filter(|&&s| s >= 80.0)
        .copied()
        .collect();
    println!("High scores: {:?}", high_scores);

    let (mean, min, max) = stats(&scores);
    println!("Mean={:.2}  Min={:.2}  Max={:.2}", mean, min, max);

    // TODO 1 output
    let med = median(&scores);
    println!("Median={:.2}", med);

    // TODO 2 output
    let (variance, std_dev) = variance_and_std_dev(&scores);
    println!("Variance={:.2}  Std Dev={:.2}", variance, std_dev);

    // --- Calling Exercise B ---
    println!("\n--- Exercise B: HashMap ---");
    hashmap::run();

    // --- Calling Exercise C ---
    println!("\n--- Exercise C: Closures & Iterators ---");
    closures_iterators::run();

    // --- Calling Exercise D ---
    println!("\n--- Exercise D: Custom Iterator ---");
    custom_iterator::run();

    //------Stretch goal--------
    println!("\n---- Running the stretch goal----");

    stretch_goal4::run();
}