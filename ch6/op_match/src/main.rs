#![allow(dead_code)]

#[derive(Debug)]
enum Estado {
    Alabama,
    Alaska,
    NewYork,
}

enum Moeda {
    Penny,
    Nickel,
    Dime,
    Quarter(Estado),
}

impl Moeda {
    fn valor_em_cents(self) -> u32 {
        match self {
            Moeda::Penny => 1,
            Moeda::Nickel => 5,
            Moeda::Dime => 10,
            Moeda::Quarter(estado) => {
                println!("Quarter do estado {:?}", estado);
                25
            },
        }
    }
}

fn main() {
    let moeda = Moeda::Quarter(Estado::Alaska);
    println!("{}", moeda.valor_em_cents());

    let cinco = Some(5);
    let seis = mais_um(cinco);
    let nenhum = mais_um(None);

    println!("{:?}, {:?}, {:?}", cinco, seis, nenhum);

    let some_value_u8 = 0u8;
    match some_value_u8 {
        1 => println!("um"),
        3 => println!("três"),
        5 => println!("cinco"),
        7 => println!("sete"),
        _ => (),
    }
}

fn mais_um(x: Option<i32>) -> Option<i32> {
    match x {
        Some(value) => Some(value + 1),
        None => None,
    }
}