// Define a Summary trait with a default implementation
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// Implement the Summary trait for NewsArticle
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Implement the Summary trait for Tweet
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Function using trait bounds to print summaries
pub fn print_summary(item: &impl Summary) {
    println!("Summary: {}", item.summarize());
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust Traits Explained"),
        location: String::from("Internet"),
        author: String::from("Rusty Coder"),
        content: String::from("Learn about Rust traits and their usage."),
    };

    let tweet = Tweet {
        username: String::from("rust_lover"),
        content: String::from("Check out this awesome Rust code!"),
        reply: false,
        retweet: true,
    };

    // Print summaries using the print_summary function
    print_summary(&article);
    print_summary(&tweet);
}
