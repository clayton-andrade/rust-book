#![allow(unused_variables)]

fn main() {
    let s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = data.to_owned();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    println!("{}", s);
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = String::from("bar");
    s1.push_str(&s2);
    println!("{}", s1);

    let s1 = "foo".to_owned();
    let s2 = "bar".to_owned();
    let s3 = s1 + &s2;
    // println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let s1 = String::from("hello");
    // let h = s1[0];

    let len = String::from("Hola").len();
    println!("Tamanho: {}", len);
    let len = String::from("Здравствуйте").len();
    println!("Tamanho: {}", len);
    let hello = String::from("Здравствуйте");
    let s = &hello[0..4];
    // let s = &hello[0..3];
    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }   
}
