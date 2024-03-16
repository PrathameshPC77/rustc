use std::ops::{Mul, Add};

// Definition of the Point struct with one generic parameter T
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Implementation of methods for Point<T>
impl<T> Point<T> {
    // Constructor method to create a new Point instance
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    // Method to mixup two Point instances with potentially different types
    fn mixup<U>(self, _other: Point<U>) -> Point<T> {
        self
    }
}

// Implementation of methods for Point<f32>
impl Point<f32> {
    // Method to calculate the distance from the origin for f32 type
    fn distance_from_origin(&self) -> f32 {
        (self.x * self.y).sqrt()
    }
}

fn main() {
    // Creating Point instances with different types
    let point1 = Point::new(3.0, 4.0);
    let point2 = Point::new(1.5, 2.5);

    // Mixing up two points with different types
    let mixed_point = point1.mixup(point2);

    // Printing the result of distance_from_origin method
    println!("Distance from origin: {}", mixed_point.distance_from_origin());
}
