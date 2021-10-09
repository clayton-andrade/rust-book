#![allow(unused_variables)]

use crate::List::{Cons, Nil};
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let csp1 = CustomSmartPointer { data: String::from("my stuff") };
    let csp2 = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");

    let a = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(a);
    println!("CustomSmartPointer dropped before the end of main.");
}
