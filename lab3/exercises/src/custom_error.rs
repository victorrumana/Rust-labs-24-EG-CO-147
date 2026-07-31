
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum AppError {
    Parse(ParseIntError),
    OutOfRange { value: i32, min: i32, max: i32 },
    EmptyInput,
    // TODO 4: Add new error variant `DivisibleByZero`
    DivisibleByZero,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
            AppError::OutOfRange { value, min, max } => {
                write!(f, "{} is not in [{}, {}]", value, min, max)
            }
            AppError::EmptyInput => write!(f, "Input was empty"),
            // TODO 4: Handle Display for DivisibleByZero
            AppError::DivisibleByZero => write!(f, "Cannot divide by zero"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

pub fn parse_and_validate(s: &str, min: i32, max: i32) -> Result<i32, AppError> {
    if s.is_empty() {
        return Err(AppError::EmptyInput);
    }
    let n: i32 = s.trim().parse()?; // `?` converts ParseIntError via From trait
    if n < min || n > max {
        return Err(AppError::OutOfRange { value: n, min, max });
    }
    Ok(n)
}

// TODO 4: Implement function `safe_div(a: i32, b: i32) -> Result<i32, AppError>`
pub fn safe_div(a: i32, b: i32) -> Result<i32, AppError> {
    if b == 0 {
        Err(AppError::DivisibleByZero)
    } else {
        Ok(a / b)
    }
}

pub fn run() {
    let test_cases = vec!["42", " 101 ", "abc", "", "-5"];
    
    println!("--- Testing Parse and Validate ---");
    for case in test_cases {
        match parse_and_validate(case, 0, 100) {
            Ok(n) => println!("Valid: {}", n),
            Err(e) => println!("Error for {:?}: {}", case, e),
        }
    }

    println!("\n--- Testing Safe Division (TODO 4) ---");
    let division_tests = vec![(10, 2), (10, 0)];
    for (a, b) in division_tests {
        match safe_div(a, b) {
            Ok(result) => println!("{} / {} = {}", a, b, result),
            Err(e) => println!("Error ({} / {}): {}", a, b, e),
        }
    }
}

fn main() {
    run();
}