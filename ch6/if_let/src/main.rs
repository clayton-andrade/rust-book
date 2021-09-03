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

fn main() {
    let some_value_u8 = Some(0u8);
    match some_value_u8 {
        Some(3) => println!("três"),
        _ => (),
    }

    if let Some(3) = some_value_u8 {
        println!("três");
    }

    let moeda = Moeda::Quarter(Estado::Alabama);

    // let mut count = 0;
    // match moeda {
    //     Moeda::Quarter(estado) => println!("Quarter do estado {:?}", estado),
    //     _ => count += 1,
    // }

    let mut _count = 0;
    if let Moeda::Quarter(estado) = moeda {
        println!("Quarter do estado {:?}", estado);
    } else {
        _count += 1;
    }
}
