use std::io;

fn main() {
    // let guess: u32 = "42".parse().expect("Not a number!");

    // Scalar type


    // floating type 
    let _x = 2.0; // f64
    let _y: f32 = 3.0;
    println!("{_x}"); // f32

    // numeric
    // addition
    let _sum = 5 + 10;
    println!("{_sum}");
    // subtraction
    let _difference = 95.5 - 4.3;
    // multiplication
    let _product = 4 * 30;
    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1
    // remainder
    let _remainder = 43 % 5;

    // boolean type 
    let _t = true;
    let _f: bool = false; // with explicit type annotation
    println!("{_t}");

    // character type 
    // let _c = '_z';
    // let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';
    println!("{_heart_eyed_cat}");
    println!("");


    // compund type 


    // tuple type - length is always fixed
    let tup = (500, 6.4, 1);
    let (_p, _q, _r) = tup;
    println!("The value of y is: {_q}");

    let _s: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = _s.0;
    let _six_point_four = _s.1;
    let _one = _s.2;
    println!("the value of one is : {_one}");
    println!("");

    // array type 
    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let _sixth_indexed = _months[6];
    println!("the sixth indexed month is : {_sixth_indexed}");
    println!("");
    
    // accessing array with an index
    let _d = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut _index = String::new();
    io::stdin()
        .read_line(&mut _index)
        .expect("Failed to read line");

    let _index: usize = _index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let _element = _d[_index];
    println!("The value of the element at index {_index} is: {_element}");
}
