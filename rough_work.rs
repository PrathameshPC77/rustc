use std::io;

fn main() {
    // Create a mutable String to store the user input
    let mut user_input = String::new();

    // Print a prompt to the user
    println!("Enter something:");

    // Read user input from the standard input (stdin)
    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line");

    // Print the user input
    println!("You entered: {}", user_input);
}
