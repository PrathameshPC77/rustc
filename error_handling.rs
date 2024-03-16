use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct CustomError {
    message: String,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CustomError {}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Try to access an element at index 10, which is out of bounds
    let result = get_element_at_index(&numbers, 10);

    match result {
        Ok(value) => println!("The value at index 10 is: {}", value),
        Err(err) => {
            // Print the custom error message
            eprintln!("Error: {}", err);

            // Print the panic message (if available)
            if let Some(panic_message) = err.source() {
                eprintln!("Panic message: {}", panic_message);
            }
        }
    }
}

fn get_element_at_index(numbers: &Vec<i32>, index: usize) -> Result<i32, Box<dyn std::error::Error>> {
    // Check if the index is within bounds
    if index < numbers.len() {
        // Return the element if the index is valid
        Ok(numbers[index])
    } else {
        // Panic with a custom error message
        Err(Box::new(CustomError {
            message: format!("Index out of bounds: {} (len: {})", index, numbers.len()),
        }))
    }
}
