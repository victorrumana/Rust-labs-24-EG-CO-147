mod enums_and_match;

mod option;

mod fizzbuzz;

mod stretch_goal2;

fn main() {
    // -- if as an expression ----------------------------------
    let number = 7;
    let description = if number % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", number, description);

    // -- loop with break value --------------------------------
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // loop returns a value
        }
    };
    println!("Loop result: {}", result);

    // -- while ------------------------------------------------
    let mut n = 1;
    while n < 100 {
        n *= 2;
    }
    println!("First power of 2 >= 100: {}", n);

    // -- for over a range -------------------------------------
    let sum: i32 = (1..=100).sum();
    println!("Sum 1..=100 = {}", sum);

    // TODO 1: Using a for loop and a range, print the
    //         multiplication table for 7 (7x1 through 7x12).
    println!("\n--- Multiplication Table for 7 ---");
    for i in 1..=12 {
        println!("7 x {} = {}", i, 7 * i);
    }

    //-----------------------------------------------------
    println!("\n--- Running Exercise B ---");
    
    enums_and_match::run(); 

    //--------------------------------------------------
    println!("\n--- Running Exercise C  ---");
    
    option::run();

    //--------------------------------------------------
    println!("\n--- Running Exercise D ---");
    
    fizzbuzz::run();

    //--------------------------------------------------
    println!("\n --- Running the stretch goal");

    stretch_goal2::run();
    
}


