fn main() {
    println!("Hello, main!");
    some_function();
    some_function_2(10);
    some_function_3(10, 20);

    let x = {
        let y = 100;
        println!("y = {}", y);
        y - 15
    };
    println!("x = {}", x);

    let cinco = retorna_cinco();
    println!("A função retorna_cinco() retornou: {}", cinco);

    let num = add_one(7);
    println!("valor da variável 'num': {}", num);

    let num = add_two(8);
    println!("valor da variável 'num': {}", num);
}

fn some_function() {
    println!("Hello, some_function!");
}

fn some_function_2(x: i32) {
    println!("O valor de x é {}", x);
}

fn some_function_3(x: i32, y: i32) {
    println!("O valor de x é {}", x);
    println!("O valor de y é {}", y);
}

fn retorna_cinco() -> i32 {
    5
}

fn add_one(num: i32) -> i32 {
    num + 1
}

fn add_two(num: i32) -> i32 {
    return num + 2;
}