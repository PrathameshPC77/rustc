fn main() {   // without borrowing
    {
        let x1 = String::from("prxthxmesh");
        let (x2, len) = calculate_length(x1);
        println!("The length of '{}' is {}.", x2, len);
    }
    
    fn calculate_length(x: String) -> (String, usize) {
        let length = x.len(); // len() returns the length of a String
        (x, length)
    }



    // with borrowing
    {
        let s1 = String::from("prxthxmesh");
        let len = calculate_length1(&s1);  // & -> reference
        println!("The length of '{}' is {}.", s1, len);
    }
    
    fn calculate_length1(s: &String) -> usize {
        s.len()
    }


    // 
    {
        let mut s2 = String::from("prxthxmesh");
        change(&mut s2);
        println!("joind string is {}.",s2)
    }
    
    fn change(some_string: &mut String) {
        some_string.push_str(" pc");
    }


    let mut g = String::from("hello");
    let r1 = &g; // no problem
    let r2 = &g; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut g; // no problem
    println!("{}", r3);



    // Dangling References
    // {
    //     let reference_to_nothing = dangle();
    // }
    // fn dangle() -> &String { // dangle returns a reference to a String
    //     let o = String::from("hello"); // s is a new String
    //     &o // we return a reference to the String, s
    // } // Here, s goes out of scope, and is dropped. Its memory goes away.
    //   // Danger!

    // fn no_dangle() -> String {
    //     let o = String::from("hello");
    //     o
    // }

}
