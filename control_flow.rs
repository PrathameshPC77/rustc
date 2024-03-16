use std::io;
use std::{thread,time};

fn main() {
    let mut number = 3;

    // if expression
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }

    // if-else with user-input
    let mut number_1 = String::new();
    println!("Enter number to check divisible by 4, 3 or 2:");
    io::stdin().read_line(&mut number_1)
        .expect("Failed to read line");
    let parsed_input: Result<i32, _> = number_1.trim().parse();
    match parsed_input {
        Ok(number_1) => {
            if number_1 % 4 == 0 {
                println!("number is divisible by 4");
            } else if number_1 % 3 == 0 {
                println!("number is divisible by 3");
            } else if number_1 % 2 == 0 {
                println!("number is divisible by 2");
            } else {
                println!("number is not divisible by 4, 3, or 2");
            }}
        Err(_) => {
            println!("Invalid input. Please enter a valid integer.");
        }
    }
    

    // condition true or false
    let condition = 3 < 2;
    let number = if condition { "condition matched" } else { "condition not matched" };
    println!("The value of number is : {number}");

    // Repeating Code with loop
    let mut p = 0;
    loop{
        println!("prxthxmesh");
        p+=1;
        println!("value of p is : {p}");

        if p >= 3{
            break;
        }
        thread::sleep(time::Duration::from_secs(1));
    }

    // Returning Values from Loops using break
    let mut counter = 0;
    let result = loop {
        println!("{counter}");
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // while loop
    let mut number_2 = 3;
    while number_2 != 0 {
        println!("{number}!");
        number_2 -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    println!("");

    // for loop
    let b = [10, 20, 30, 40, 50];
    for element in b {
        println!("the value is: {element}");
    }
    println!("");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}