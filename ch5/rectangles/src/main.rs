use std::fmt;

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn new(length: u32, width: u32) -> Self {
        Rectangle {
            length,
            width,
        }
    }

    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Length: {}, Width: {}]", self.length, self.width)
    }
}

fn main() {
    let length = 50;
    let width = 30;

    // let rect = (length, width);

    let rect = Rectangle { length, width };

    println!("The area of the rectangle is {} square pixels.", rect.area());
    println!("{:?}", rect);
    println!("{}", rect);

    let rect1 = Rectangle::new(50, 30);
    let rect2 = Rectangle::new(40, 10);
    let rect3 = Rectangle::new(45, 60);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// fn area(length: u32, width: u32) -> u32 {
//     length * width
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(rect: &Rectangle) -> u32 {
//     rect.length * rect.width
// }