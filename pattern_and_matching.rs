// Define an enum named Coin
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),  // Variant Quarter includes a UsState value
}

// Define an enum named UsState
#[derive(Debug)] // Derive the Debug trait for easy inspection
enum UsState {
    Alabama,
    Alaska,
    // Add other states as needed
}

// Define a function value_in_cents that takes a Coin enum and returns a u8
fn value_in_cents(coin: Coin) -> u8 {
    // Use match to compare the coin against its variants and execute code accordingly
    let result = match coin {
        Coin::Penny => {
            println!("Penny!");
            1
        }
        Coin::Nickel => {
            println!("Nickel!");
            5
        }
        Coin::Dime => {
            println!("Dime!");
            10
        }
        Coin::Quarter(state) => {
            // Use a variable binding to extract the UsState value from the Quarter variant
            println!("State quarter from {:?}!", state);
            25
        }
    };
    println!("Result: {}", result);
    result
}

// Define a function plus_one that takes an Option<i32> and returns an Option<i32>
fn plus_one(x: Option<i32>) -> Option<i32> {
    // Use match to handle the Some and None variants of Option<i32>
    let result = match x {
        None => {
            println!("None case!");
            None
        }
        Some(i) => {
            let new_value = i + 1;
            println!("Adding one: {} + 1 = {}", i, new_value);
            Some(new_value)
        }
    };
    println!("Result: {:?}", result);
    result
}

// Use match to implement a dice rolling game logic
fn main() {

    let penny_result = value_in_cents(Coin::Penny);
    println!("Result for Penny: {}", penny_result);

    // let nickel_result = value_in_cents(Coin::Nickel);
    // println!("Result for Nickel: {}", nickel_result);

    // let dime_result = value_in_cents(Coin::Dime);
    // println!("Result for Dime: {}", dime_result);

    let quarter_result = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("Result for Quarter: {}", quarter_result);



    let dice_roll = 9;
    match dice_roll {
        3 => {
            println!("Adding fancy hat!");
            add_fancy_hat();
        }
        7 => {
            println!("Removing fancy hat!");
            remove_fancy_hat();
        }
        _ => {
            println!("Rerolling!");
            reroll();
        }
    }
}

// Functions representing different actions in the game
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
