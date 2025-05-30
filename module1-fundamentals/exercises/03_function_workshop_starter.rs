// a function that adds two integers and returns the result
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// a function that calculates the area of a rectangle
fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    return width*height;
}

// TODO: 3. Define a function that checks if a number is prime
fn is_prime(number: u32) -> bool {
    // TODO: Implement the prime number check logic
    //       A number is prime if it's greater than 1 and 
    //       only divisible by 1 and itself
    if number <= 1  {
        return false;
    }
    for i in 2..=(number as f64).sqrt() as u32 {
        if number % i == 0 {
            return false;
        }
    }
    return true;
}

// Define a function that converts Fahrenheit to Celsius
// Formula: C = (F - 32) * 5/9
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    
    let celsusius = (fahrenheit - 32.0) * 5.0 / 9.0;
    return celsusius;
    
}

fn main() {
    let sum1 = add(20, 30);
    let sum2 = add(10, 25);
    

    let area1 = calculate_rectangle_area(5.0, 10.0);
    let area2 = calculate_rectangle_area(20.10, 15.5);
    
    let prime_check1 = is_prime(7);
    let prime_check2 = is_prime(400);
    
    let celsius1 = fahrenheit_to_celsius(98.6);
    let celsius2 = fahrenheit_to_celsius(32.0);
    
    
    println!("Sum of 20 and 30 is: {}", sum1);
    println!("Sum of 10 and 25 is: {}", sum2);
    println!("Area of rectangle with width 5 and height 10 is: {} square units", area1);
    println!("Area of rectangle with width 20.10 and height 15.5 is: {} square units", area2);
    println!("Is 7 a prime number? {}", prime_check1);
    println!("Is 400 a prime number? {}", prime_check2);
    println!("32.0°F is equivalent to {}°C", celsius2);
    println!("98.6°F is equivalent to {}°C", celsius1);
}