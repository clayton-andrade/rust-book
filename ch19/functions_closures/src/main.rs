#![allow(dead_code)]

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let _list_of_strings = list_of_numbers.iter().map(|n| n.to_string()).collect::<Vec<String>>();

    let list_of_numbers = vec![1, 2, 3];
    let _list_of_strings = list_of_numbers.iter().map(ToString::to_string).collect::<Vec<String>>();

    let _list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    let _list_of_statuses: Vec<Status> = (0u32..20).map(|v| Status::Value(v)).collect();
    for status in _list_of_statuses.iter() {
        println!("{:?}", status);
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
