fn main() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("{}ª iteração", counter);
        if counter == 20 {
            break;
        }
    }

    let mut n = 10;
    while n >= 0 {
        println!("n = {}", n);
        n -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index: usize = 0;
    while index < 5 {
        println!("O valor de a[{}] é {}", index, a[index]);
        index += 1;
    }

    let b = [100, 200, 300, 400, 500];
    for i in 0..b.len() {
        println!("O valor de a[{}] é {}", i, b[i]);
    }
    for elemento in b.iter() {
        println!("O valor é {}", elemento);
    }

    for numero in 0..6 {
        print!("{} ", numero)
    }
    println!();
    for numero in (0..=5).rev() {
        print!("{} ", numero)
    }
    println!();
}
