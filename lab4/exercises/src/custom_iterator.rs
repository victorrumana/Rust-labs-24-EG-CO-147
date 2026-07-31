pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(self.a)
    }
}

// TODO 6: Custom Primes iterator
pub struct Primes {
    current: u64,
}

impl Primes {
    pub fn new() -> Self {
        Primes { current: 2 }
    }

    fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2..=((n as f64).sqrt() as u64) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while !Self::is_prime(self.current) {
            self.current += 1;
        }
        let prime = self.current;
        self.current += 1;
        Some(prime)
    }
}

pub fn run() {
    let fibs: Vec<u64> = Fibonacci::new().take(15).collect();
    println!("First 15 Fibonacci: {:?}", fibs);

    // TODO 5: Find the first Fibonacci number greater than 1,000,000
    let first_over_1m = Fibonacci::new()
        .find(|&x| x > 1_000_000)
        .unwrap();
    println!("First Fibonacci number > 1,000,000: {}", first_over_1m);

    // TODO 6: First 10 primes using custom Primes iterator
    let first_10_primes: Vec<u64> = Primes::new().take(10).collect();
    println!("First 10 Primes: {:?}", first_10_primes);
}