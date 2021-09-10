#![allow(unused_variables)]

fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let mut v3 = Vec::new();
    v3.push(10);
    v3.push(20);
    v3.push(30);

    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("Terceiro elemento: {}", third);
    let third = v.get(2);
    println!("Terceiro elemento: {:?}", third);
    // let does_not_exist = &v[10];
    // println!("{}", does_not_exist);
    let does_not_exist = v.get(10);
    println!("{:?}", does_not_exist);

    let v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6);
    println!("{:?}", first);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue"))];
    println!("{:?}", row);

    let mut v = (0..100).collect::<Vec<i32>>();
    while let Some(i) = v.pop() {
        println!("{}", i);
    }
    println!("{:?}", v);
}
