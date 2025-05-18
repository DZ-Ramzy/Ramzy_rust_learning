use std::io::{self, Write};

fn main() {
    // Part 1: FizzBuzz Implementation
    println!("=== FizzBuzz Challenge ===");
    
    for i in 1..=20 {

        if i%3 ==0 && i%5 ==0 {
            println!("FizzBuzz");
        }
        else if i%3 ==0 {
            println!("Fizz");
        }
        else if i%5 ==0 {
            println!("Buzz");
        }
        else {
            println!("{}", i);
        }
    }
    
    // Part 2: Menu-driven Calculator
    println!("\n=== Calculator ===");
    
    // TODO: Create a variable to control the calculator loop
    let mut running = true;
    
    // Implement the calculator loop
    while running {
        // Show the menu options
        println!("Choose an operation:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");
        
        // Get the user's choice
        print!("Enter your choice : ");
        io::stdout().flush().unwrap(); // flush the output buffer to ensure the prompt appears before user input
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line :( ");
        
        // Convert choice to a number (with error handling)
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
        
        // Exit if the user chose option 5
        if choice == 5 {
            // TODO: Set running to false to exit the loop
            running = false;
            println!("Exiting the calculator. Goodbye!");
        }
        
        print!("Enter the first number: ");
        io::stdout().flush().unwrap();
        let mut first_number = String::new();
        io::stdin().read_line(&mut first_number).expect("Failed to read line :( ");


        print!("Enter the second number: ");
        io::stdout().flush().unwrap();
        let mut second_number = String::new();
        io::stdin().read_line(&mut second_number).expect("Failed to read line :( ");
        
        match choice {
            1 => println!("Result: {}", first_number.trim().parse::<f64>().unwrap() + second_number.trim().parse::<f64>().unwrap()), // Handle addition
            2 => println!("Result: {}", first_number.trim().parse::<f64>().unwrap() - second_number.trim().parse::<f64>().unwrap()), // Handle subtraction
            3 => println!("Result: {}", first_number.trim().parse::<f64>().unwrap() * second_number.trim().parse::<f64>().unwrap()), // Handle multiplication
            4 => println!("Result: {}", first_number.trim().parse::<f64>().unwrap() / second_number.trim().parse::<f64>().unwrap()), // Handle division
            _ => println!("Invalid option. Please try again."),
        }
        
        print!("Do you want to perform another calculation? (y/n): ");
        io::stdout().flush().unwrap();
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read line :( ");
        
        if response.trim().to_lowercase() != "y" {
            running = false;
            println!("Exiting the calculator. Goodbye!");
        }
    }
    
    println!("Thank you for using the calculator!");
}