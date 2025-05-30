use std::io::{self, Write};
use chrono::{DateTime, Local};

fn main() {
    // Get the user's name

    print!("Hey, what is your name? ");
    io::stdout().flush().unwrap(); // flush the output buffer to ensure the prompt appears before user input
    
    // TODO: 2. Read the user's input
    let mut name = String::new();   
    io::stdin().read_line(&mut name).expect("Failed to read line :( ");


    // personalize the greeting

    println!("Nice!, welcome to the Bootcamp, {}!", name.trim());

    // Print the current date

    let local: DateTime<Local> = Local::now();
    println!("Current date and time: {}", local.format("%Y-%m-%d %H:%M:%S"));
}