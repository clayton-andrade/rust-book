const MAX_POINTS: i32 = 1_000_000;

fn main() {
    let mut x = 5;
    println!("O valor de x é: {}", x);
    x = 6;
    println!("O valor de x é: {}", x);

    println!("MAX_POINTS = {}", MAX_POINTS);

    // Shadowing (sombreamento de variáveis)
    let y = 10;
    println!("O valor de y é: {}", y);
    let y = y + 10;
    println!("O valor de y é: {}", y);
    let y = 50;
    println!("O valor de y é: {}", y);

    // Sombreamento com alteração de tipo
    let name = "Clayton";
    println!("Valor de 'spaces': {}", name);
    let name = name.len();
    println!("Valor de 'spaces': {}", name);

    // Caso em que o tipo de dados tem que ser especificado (a inferência de tipo não funciona)
    let _var = "33".parse::<i32>().expect("Não foi possível converter");
    let _var: u128 = "33".parse().expect("Não foi possível converter");

    // Caso em que a inferência de tipo funciona
    let _var = "variável".len();

    // Tipos inteiros
    let int1 = 4367;
    let int2: i64 = 58;
    let int3 = 200u8;
    let int4: usize = "int4".len();
    println!("Tipos inteiros: {} - {} - {} - {}", int1, int2, int3, int4);

    // Tipos de ponto flutuante
    let float1 = 23.7;
    let float2 = 8.77f64;
    let float3: f32 = 10.0;
    println!("Tipos de ponto flutuante: {} - {} - {}", float1, float2, float3);

    // Tipo booleano
    let _bool1 = true;
    let _bool2: bool = false;

    // Tipo caractere
    let c = 'c';
    let c_upper: char = 'C';
    println!("Caracteres: '{}', '{}'", c, c_upper);

    // Tipo tupla
    let tup1 = (1, 5.4, 7);
    let tup2: (u8, i32, char) = (121, -478, 'A');

    // Desestruturação de tuplas
    let (a, b, c) = tup1;
    println!("{}, {}, {}", a, b, c);
    let (d, _, f) = tup1;
    println!("{}, {}", d, f);

    // Acessando valores através de índices
    let a = tup2.0;
    let b = tup2.1;
    let c = tup2.2;
    println!("{}, {}, {}", a, b, c);

    // Tipo array / matriz
    let numeros = [1, 2, 3, 4, 5];
    let letras: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    let meses = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho", "Julho",
                  "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"];

    let letra1 = letras[0];
    let num2 = numeros[1];

    println!("{} - {}", letra1, num2);
    println!("{:?}", meses);
}
