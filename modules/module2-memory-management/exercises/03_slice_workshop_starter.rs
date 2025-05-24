// 1. Find the first word in a string
fn first_word(s: &str) -> &str  {
    // Return the first word in the string (up to the first space or the entire string if no spaces)
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s // Return the entire string if no spaces are found
}


// 2. Calculate the sum of elements in an array slice
fn sum_slice(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

// 3. Find the middle element(s) of a slice
fn middle_elements<T>(slice: &[T]) -> &[T] {
    // Return the middle element if length is odd, or the two middle elements if length is even
    if slice.len() % 2 == 1 {
        let mid = slice.len() / 2;
        &slice[mid..mid + 1]
    } else {
        let mid = slice.len() / 2;
        &slice[mid - 1..mid + 1]
    }
}

// 4. Extract a subslice based on a condition (e.g., all positive numbers)
fn extract_positive(numbers: &[i32]) -> &[i32] {
    // Find the start of the first positive sequence
    let mut start = 0;
    while start < numbers.len() && numbers[start] <= 0 {
        start += 1;
    }
    
    // If no positive numbers found, return empty slice
    if start == numbers.len() {
        return &numbers[0..0];
    }
    
    // Find the end of the positive sequence
    let mut end = start;
    while end < numbers.len() && numbers[end] > 0 {
        end += 1;
    }
    
    &numbers[start..end]
}

fn main() {
    // 1. Test first_word function
    let sentence = String::from("Hello Rust slices world");
    let first = first_word(&sentence);
    println!("First word: {}", first);
    
    let empty_str = String::from("");
    let first_empty = first_word(&empty_str);
    println!("First word in empty string: '{}'", first_empty);
    
    // 2. Test sum_slice function
    let numbers = [1, 2, 3, 4, 5];
    let sum = sum_slice(&numbers);
    println!("Sum of all elements: {}", sum);
    
    let partial_sum = sum_slice(&numbers[0..3]);
    println!("Sum of first three elements: {}", partial_sum);
    
    // 3. Test middle_elements function
    let vec1 = vec![1, 2, 3, 4, 5]; // Odd length
    let mid1 = middle_elements(&vec1);
    println!("Middle element(s) of odd-length vector: {:?}", mid1);
    
    let vec2 = vec![1, 2, 3, 4]; // Even length
    let mid2 = middle_elements(&vec2);
    println!("Middle element(s) of even-length vector: {:?}", mid2);
    
    // 4. Test extract_positive function
    let mixed_numbers = [3, 5, 2, -1, -5, 8, 10, -3];
    let positive_run = extract_positive(&mixed_numbers);
    println!("First run of positive numbers: {:?}", positive_run);
    
    let negative_start = [-2, -5, 3, 4, 5, -1, 7];
    let positive_run2 = extract_positive(&negative_start);
    println!("First run of positive numbers (starting negative): {:?}", positive_run2);
}