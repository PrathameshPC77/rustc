fn find_index(vec: Vec<i32>, element: i32)-> Option<usize>{
    for (index,value) in vec.iter().enumerate(){
        if *value == element{
            return Some(index);
        }  
    }
    None
}

fn main() {
    // println!("Hello, world!");

    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    
    // let some_number = Some(5);
    // let some_char = Some('e');

    // let absent_number: Option<i32> = None;

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // // let sum = x + y;


    // example
    let numbers= vec![1,2,3,4,5];
    let index = find_index(numbers, 6);

    match index{
        Some(i)=> println!("element found at index {}",i),
        None=> println!("element not found")
    }
}

