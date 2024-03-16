fn main() {
    let my_string:String=String::from("hello prxthxmesh welcome to slice type");
    let hello = &my_string[0..5];  //slice
    let world = &my_string[6..16];
    println!("{hello}");
    println!("{world}");
    
    let x = first_word(&my_string);
    // my_string.clear();
    println!("first word of my_string is {x}")



    // let a = [1, 2, 3, 4, 5];
    // let slice= &a[1..3];
    // assert_eq!(slice, &[2, 3]);

    // println!("{slice}")
    
}

fn first_word(_s: &String) -> &str{
    let bytes = _s.as_bytes();
    // println!("{:?}",bytes);
    // println!("{}",b' ');

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &_s[0..i];
        }
    }
   & _s[..] 
}


