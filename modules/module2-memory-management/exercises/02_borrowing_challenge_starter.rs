// 1. Processing string data with immutable references
fn get_length(text: &str) -> usize {
    // Return the length of the string
    return text.len()
}

// 2. Modifying vector data with mutable references
fn add_three_elements(numbers: &mut Vec<i32>) {
    // Add three elements (10, 20, and 30) to the vector
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
}

// 3. Processing multiple data structures
fn calculate_stats(numbers: &Vec<f64>, strings: &Vec<String>) -> (f64, i32) {
    // Calculate and return the average of the numbers vector and the count of items in the strings vector
    let mut sum: f64 = 0.0;
    let mut count: i32 = 0;
    let size = numbers.len();

    for i in 0..size{
        sum += numbers[i];
    } 

    let average = sum / size as f64;

    let size_strings = strings.len();

    for _i in 0..size_strings{
        count += 1;
    }

    return (average, count);
}

// 4. Borrowing rules demonstration
fn fix_borrowing_issues() {
    let mut data = vec![1, 2, 3];
    
    // The following code has borrowing issues. Uncomment and fix it.
    { 
        let ref1 = &mut data;
        ref1.push(4);
    }

    {
        let ref2 = &mut data;
        ref2.push(5);
    }

    println!("Modified data: {:?}", data);
    
    //  Fix another example of borrowing issue
    {
        let ref3 = &data;
        println!("Data length: {}", ref3.len());
    }

    {
        let ref4 = &mut data;
        ref4.push(6);
    }

    println!("Modified data: {:?}", data);

}

fn main() {
    // 1. Test immutable reference function
    let test_string = String::from("Hello, Rust borrowing!");
    let length = get_length(&test_string);
    println!("String length: {}", length);
    // Verify the string is still usable after passing as reference
    println!("Original string: {}", test_string);
    
    // 2. Test mutable reference function
    let mut my_vec: Vec<i32> = Vec::new();
    println!("Before function call: {:?}", my_vec);
    add_three_elements(&mut my_vec);
    println!("After function call: {:?}", my_vec);
    
    // 3. Test multiple references
    let numbers = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    let words = vec![String::from("apple"), String::from("banana"), String::from("cherry")];
    let (average, count) = calculate_stats(&numbers, &words);
    println!("Average of numbers: {:.1}, Count of strings: {}", average, count);
    
    // 4. Test the fixed borrowing issues
    fix_borrowing_issues();
}