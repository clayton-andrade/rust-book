#![allow(unused_variables)]

use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn main() -> Result<(), io::Error> {
    // panic!("Quebra tudo!!!");
    // let v = vec![1, 2, 3];
    // v[99];

    // let f = match File::open("hello.txt") {
    //     Ok(file) => file,
    //     Err(error) => {
    //         match error.kind() {
    //             ErrorKind::NotFound => {
    //                 match File::create("hello.txt") {
    //                     Ok(file) => file,
    //                     Err(error) => panic!("Erro na criação do arquivo: {}", error),
    //                 }
    //             },
    //             _ => panic!("Erro ao abrir o arquivo: {}", error),
    //         }
    //     },
    // }; 
    
    let f = match File::open("hello.txt") {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(f) => f,
                Err(e) => panic!("Erro na criação do arquivo: {}", e),
            }
        },
        Err(e) => panic!("Erro ao abrir o arquivo: {}", e),
    };

    // let f = File::open("world.txt").unwrap();
    // let f = File::open("world.txt").expect("Erro ao abrir o arquivo");

    // let username = match read_username_from_file("hello.txt") {
    //     Ok(s) => println!("username: {}", s),
    //     Err(e) => panic!("Error {}", e),
    // };

    let username = read_username_from_file("hello.txt")?;
    println!("username: {}", username);

    Ok(())
}

fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
    let mut s = String::new();

    File::open(filename)?.read_to_string(&mut s)?;

    if !s.is_empty() {
        return Ok(s[..(s.len() - 1)].to_owned());
    }

    Ok(s)
}
