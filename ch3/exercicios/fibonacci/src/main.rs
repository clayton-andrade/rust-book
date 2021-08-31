use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        panic!("Missing arguments.");
    }
    if let Ok(n) = args[1].parse::<i32>() {
        println!("{}", fibonacci(n));
    } else {
        panic!("Invalid number.");
    }
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
