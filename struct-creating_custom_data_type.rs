use std::fmt::Debug;

#[derive(Debug)]
#[warn(dead_code)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    mobile_number: u64,
}

fn main() {
    println!("Hello, world!");

    // numeric types
    let age: u32 = 30;
    let temperature =98.6;
    let is_rust_awesome = true;
    let first_initial = 'J';

    //compound type
    let tuple = (42, 3.14, "Hello prxthxmesh");
    let array: [u32; 3] = [10,20,30]; 


    // custom type 
    let _user1 = build_user(String::from("prxthxmesh1.com"),String::from("_user1"));
    let user2 = User{
        active: false,
        email: String::from("prxthxmesh2.com"),
        username: String::from("prxthxmesh2"),
        .._user1
    };
    let user3 = User{
        email: String::from("prxthxmesh3.com"),
        username: String::from("prxthxmesh3"),
        ..user2
    };

    println!("_user1 info : {:#?}",_user1);
    println!("user2 info : {:#?}",user2);
    println!("user3 info : {:#?}",user3);
    
    
}

fn build_user(email: String, username: String) -> User{
    // custom data type
    let _user1 = User {
        active: true,
        username: String::from("prxthxmesh1"),
        email: String::from("prxthxmesh1.com"),
        sign_in_count: 1,
        mobile_number : 8104990000,
    };
    _user1

    // let user2 = User {
    //     active: false,
    //     username: String::from("prxthxmesh2"),
    //     email: String::from("prxthxmesh2.com"),
    //     sign_in_count: 2,
    //     mobile_number : 8104900000,
    // };
    // user2
}