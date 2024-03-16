#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method to calculate the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method to check if one rectangle can hold another
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function (constructor) to create a square rectangle
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Creating an instance of Rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Calling the area method on the Rectangle instance
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    // Creating another rectangle
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    // Creating one more rectangle
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // Using the can_hold method to check if one rectangle can hold another
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Using the associated function to create a square rectangle
    let square_rect = Rectangle::square(10);
    println!("Square rectangle: {:#?}", square_rect);
}
