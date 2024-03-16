use std::u32;

struct Circle {
    pi:f32,
    radius:f32,
}

#[derive(Debug, Clone)]
struct AadharCard{
    name:String,
    gender:String,
    date_of_birth:String,
    aadhar_number:String,
    address:String,
}


fn main() {
    println!("Hello, world!");

    let base1 = 30;
    let height1 = 50;
    println!("the area of traingle is {} square pixels.",area_of_traingle(base1,height1));

    // Refactoring with Tuples
    let height_and_width = (10, 30);
    println!("The area of the rectangle is {} square pixels.",area_of_rectanle(height_and_width));
    
    let circle=Circle{
        pi:3.14,
        radius:4.0,
    };
    println!("the area of circle is : {}",area_of_circle(&circle));

    let mut prxthxmesh=AadharCard{
        name:String::from("prxthxmesh"),
        gender:String::from("male"),
        date_of_birth:String::from("11/08/2002"),
        aadhar_number:String::from("973364772972933"),
        address:String::from("navi-mumbai"),
    };
    println!("The updated aadhar address is : {}",prxthxmesh.address);
    println!("aadhar user prxthxmesh : {:#?}",prxthxmesh);

    let dhruv = update_aadhar(&mut prxthxmesh);
    println!("Original AadharCard: {:#?}", prxthxmesh);
    println!("Updated AadharCard: {:#?}", dhruv);

}

fn area_of_traingle(base1: u32,height1:u32)->u32{
    (base1*height1)/2
}

fn area_of_rectanle(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_of_circle(circle:&Circle)->f32{
    circle.pi*circle.radius*circle.radius
}

fn update_aadhar(user: &mut AadharCard) {
    user.address = String::from("kalyan");
}

