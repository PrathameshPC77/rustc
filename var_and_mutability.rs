// variables and mutability

fn main() {

    // constant if you want in program
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = 6;
    println!("The value of x is : {x}");

    let mut y = 11;
    println!("The value of y is : {}",y);
    y = 51;
    println!("The value of y is : {y}");

    {
        let y = 8;
        println!("The value of y is : {y}");
    }

    println!("The value of y is : {y}");

    let z = 21;
    println!("The value of z is : {}",z);
    // z = 28;
    // println!("The value of z is : {z}");

    // shadowing
    {
        let z =57;
        println!("The value of z is : {z}");
    }

    println!("The value of z is : {}",z);

    let _prxthxmesh = 57;
    println!("The value of prxthxmesh is : {_prxthxmesh}");
    
    println!("The value of const is : {THREE_HOURS_IN_SECONDS}");
}