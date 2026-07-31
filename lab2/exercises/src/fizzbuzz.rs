pub fn fizzbuzz_calc(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        (_, _) => n.to_string(),
    }
}

pub fn run() {
    for i in 1..=50 {
        println!("{}", fizzbuzz_calc(i));
    }
}