fn main() {
    let num = 7;
    if num < 5 {
        println!("condição era verdadeira");
    } else {
        println!("condição era falsa");
    }

    let condition = 7 > 5;
    if condition {
        println!("condição era verdadeira");
    } else {
        println!("condição era falsa");
    }

    if num != 0 {
        println!("o valor da variável 'num' é diferente de zero");
    }

    let num = 6;
    if num % 4 == 0 {
        println!("o número é divisível por 4");
    } else if num % 3 == 0 {
        println!("o número é divisível por 3");
    } else if num % 2 == 0 {
        println!("o número é divisível por 2");
    } else {
        println!("o número não é divisível por 4, 3 ou 2");
    }

    let num = if condition {
        10
    } else {
        20
    };
    println!("número = {}", num);
}
