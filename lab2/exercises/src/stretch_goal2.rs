use std::env;

pub fn evaluate_expression(expr: &str) -> Result<f64, String> {
    let parts: Vec<&str> = expr.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Invalid expression format. Use format like '3 + 4'".to_string());
    }

    let num1: f64 = parts[0].parse().map_err(|_| "Invalid first number")?;
    let operator = parts[1];
    let num2: f64 = parts[2].parse().map_err(|_| "Invalid second number")?;

    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Division by zero error".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err(format!("Unsupported operator: {}", operator)),
    }
}

pub fn run() {
    println!("=== LAB 2 STRETCH GOAL ===");
    let args: Vec<String> = env::args().collect();
    
    // Uses command line arg if provided (e.g. cargo run -- "3 + 4"), or defaults to "3 + 4"
    let expr = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "3 + 4".to_string()
    };

    println!("Evaluating expression: '{}'", expr);
    match evaluate_expression(&expr) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}