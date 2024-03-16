// static mut COUNT: u32 =0;
// const PI:f32 =3.14159;

// fn main(){
//     //accessing static variable 
//     unsafe{
//         println!("Initial count : {}",COUNT);
//         COUNT+=1;
//         println!("Updated count: {}",COUNT);
//     }
//     //calling a fn that modifies the static variable
//     increment_count();
//     //accessing static variable again
//     unsafe{
//         println!("Final count : {}",COUNT);
//     }
//     println!("Area of circle with radius 10 is {}",area_of_circle(10.0));
// }

// fn increment_count(){
//     unsafe{
//         COUNT+=1;
//     }
// }

// fn area_of_circle(radius:f32)->f32{
//     PI * radius*radius
// }




//stack allocated memory - FIFO
// fn main(){
//     let a=10; //stack allocated variable
//     let b=20;
//     //stack allocated sum to store
//     let sum=add_numbers(a,b);
//     println!("Sum : {}",sum);
// }

// fn add_numbers(x: i32,y:i32)->i32{
//     //stack allocated result to store
//     let result=x+y;
//     result
// }

// use std::result;




// heap alllocated memory
fn main(){
    let a=10; //stack allocated
    let b=20;

    let sum=add_numbers(a,b);

    let heap_value=String::from("Hello, world!");
    // heap allocated string
    println!("Sum : {}",sum); 
    println!("Heap value : {}",heap_value);
}

fn add_numbers(x:i32,y:i32)->i32{
    let result=x+y; //stack-allocated
    let heap_value2=Box::new(vec![1,2,3,4,5]);
    //heap allocated
    println!("Heap value 2 : {:?}",*heap_value2);
    result
}

