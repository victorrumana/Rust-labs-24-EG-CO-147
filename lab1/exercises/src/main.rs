// src/main.rs

mod ownership; 
mod functions;
mod stretch_goal1;

fn main() {
    
    // EXERCISE B (Variables, Mutability & Shadowing)

    let x = 5;
    println!("x = {}", x);

    let mut y = 10;
    println!("y before = {}", y);
    y += 5;
    println!("y after = {}", y);

    let pi: f64 = 3.14159;
    let is_learning: bool = true;
    let grade: char = 'A';

    println!("pi = {}", pi);
    println!("is_learning = {}", is_learning);
    println!("grade = {}", grade);

    let z = "42";                                    
    let z: u32 = z.parse().expect("Not a number!"); 
    println!("Parsed z = {}", z);


    // ==========================================
    // EXERCISE C (Ownership & Borrowing)

    println!("\n--- Running Exercise C ---");
    
    ownership::run(); 

    // =================================
    // EXERCISE D (Functions)

    println!("running exercise D");

    functions::run();

    //========================================
    println!("Running the stretch goal!");

    stretch_goal1::run();
}