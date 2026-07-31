pub fn apply_twice<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(x))
}

pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn run() {
    let double = |x| x * 2;
    let add10 = make_adder(10);
    println!("apply_twice(double, 3) = {}", apply_twice(double, 3));
    println!("add10(5) = {}", add10(5));

    // Chained iterators
    let result: Vec<String> = (1..=20)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .take(5)
        .map(|x| format!("{:4}", x))
        .collect();
    println!("First 5 even squares: {}", result.join(""));

    // fold (reduce)
    let product: u64 = (1..=10).fold(1, |acc, x| acc * x);
    println!("10! = {}", product);

    // TODO 4a: Sum of squares of odd numbers from 1 to 99
    let sum_odd_squares: i32 = (1..100)
        .filter(|&x| x % 2 != 0)
        .map(|x| x * x)
        .sum();
    println!("Sum of squares of odd numbers (1..99): {}", sum_odd_squares);

    // TODO 4b: Collect all prime numbers up to 50 into a Vec<u32>
    let primes: Vec<u32> = (1..=50)
        .filter(|&x| is_prime(x))
        .collect();
    println!("Primes up to 50: {:?}", primes);
}