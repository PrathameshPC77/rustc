enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... (other states)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    // Example usage of value_in_cents with different coins
    let penny_result = value_in_cents(Coin::Penny);
    println!("Result for Penny: {}", penny_result);

    // let nickel_result = value_in_cents(Coin::Nickel);
    // println!("Result for Nickel: {}", nickel_result);

    // let dime_result = value_in_cents(Coin::Dime);
    // println!("Result for Dime: {}", dime_result);

    let quarter_result = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("Result for Quarter: {}", quarter_result);

    // Example of if let usage
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // Example of if let with else
    let mut count = 0;
    let coin = Coin::Penny; // Assume some coin value
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }







    
    // getting point by myself what i understood :))
    // Simple if-else statement to check if a number is odd or even
    let number = 42;
    if number % 2 == 0 {
        println!("Using if-else: The number {} is even.", number);
    } else {
        println!("Using if-else: The number {} is odd.", number);
    }

    // Match expression to check if a number is odd or even
    match number % 2 {
        0 => println!("Using match: The number {} is even.", number),
        1 => println!("Using match: The number {} is odd.", number),
        _ => println!("Using match: Unexpected result"),
    }

    // If let expression to check if a number is even
    let some_value = Some(number);
    if let Some(value) = some_value {
        if value % 2 == 0 {
            println!("Using if let: The number {} is even.", value);
        } else {
            println!("Using if let: The number {} is odd.", value);
        }
    }

    // If let with else expression to check if a number is even
    let another_value = Some(33);
    if let Some(value) = another_value {
        println!("Using if let with else: Processing value {}", value);
    } else {
        println!("Using if let with else: No value present.");
    }



}
