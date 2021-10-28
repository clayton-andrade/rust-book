use std::ops::Add;
use std::fmt;

struct Meters(u32);

#[derive(Debug, PartialEq)]
struct Milimeters(u32);

impl Add<Meters> for Milimeters {
    type Output = Milimeters;

    fn add(self, other: Meters) -> Self::Output {
        Milimeters(self.0 + (other.0 * 1000))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;
    println!("{:?}, {:?}, {:?}", p1, p2, p3);
    assert_eq!(p1 + p2, Point {x: 3, y: 3 });

    let m = Meters(2);
    let mm = Milimeters(287);
    assert_eq!(mm + m, Milimeters(2287));

    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    p2.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
