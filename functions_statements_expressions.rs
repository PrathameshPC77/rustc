fn main() {
    println!("Hello, world!");

    another_function();

    //with parameters
    another_function_with_parameter(11);

    print_labeled_measurement(5, 'h');

    sum_of_x_and_y(8,51);
    sum_of_x_and_y(11,57);


    //statement and expressions
    let y={
        let x =50;
        x+1
    };   // here y is an expression and x is a statement
    println!("the value os y is : {y}");

    let x = five();
    println!("The value of x is : {x}");

    let z = plus_one(8);
    println!("the value of z is : {z}");

}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_parameter(x: i32){
    println!("The value of x is : {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32{
    5
}

fn plus_one(z:i32) -> i32{
    z+1  // no ; 
}

fn sum_of_x_and_y(x:i32,y:i32){
    let sum = x + y;
    println!("the sum of x and y is : {sum}")

}