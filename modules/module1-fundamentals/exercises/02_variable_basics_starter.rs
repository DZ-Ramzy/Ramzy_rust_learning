fn main() {
    // Declare an immutable integer variable
    let my_integer = 42;

    // Declare a mutable float variable and modify it later
    let mut my_float = 3.14;
    
    // Modify the float value
    let old_float = my_float;
    my_float = 6.28;
    
    // Declare a boolean variable using type inference
    let is_rust_fun = true;
    
    // Declare a character variable with explicit type annotation
    let my_char: char = 'R';
    
    // TODO: 5. Perform arithmetic operations with the numeric variables
    let sum = my_integer as f64 + old_float; // Convert integer to float for addition
    let product:f64 = my_integer as f64* old_float; // Convert float to integer for multiplication
    
    // TODO: 6. Print all variables and calculation results with appropriate labels
    println!("Integer value: {}", my_integer);
    println!("Original float value: {}", old_float);
    println!("Modified float value: {}", my_float);
    println!("Boolean value: {}", is_rust_fun);
    println!("Character value: {}", my_char);
    println!("Addition result: {}", sum);
    println!("Multiplication result: {}", product);
}