use std::fmt::Display;

// Struct with a reference field
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Methods for the ImportantExcerpt struct
impl<'a> ImportantExcerpt<'a> {
    // Method with no reference return type
    fn level(&self) -> i32 {
        3
    }

    // Method with reference return type and lifetime elision
    fn announce_and_return_part(&self, announcement: &str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Function with generic type parameter, trait bounds, and lifetimes
fn longest_with_an_announcement<'a, T>(x: &'a str,y: &'a str,ann: T,)-> &'a str where T: Display,{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // Example of using the ImportantExcerpt struct
    let novel = String::from("Call me prxthxmesh. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence,};

    // Example of using the longest_with_an_announcement function
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest_with_an_announcement(string1.as_str(), string2, "Important!");
    println!("The longest string is {}", result);

    // Example of using methods on the ImportantExcerpt instance
    println!("Level: {}", i.level());
    println!("Part: {}", i.announce_and_return_part("sharvin"));
}
