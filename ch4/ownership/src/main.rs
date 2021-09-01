#![allow(unused_variables)]

fn main() {
    let outside_s = String::from("texto 1");
    {
        let inside_s = String::from("texto 2");
        println!("{}", outside_s);
        println!("{}", inside_s);
    } // Fim do escopo de inside_s
    println!("{}", outside_s);
    // println!("{}", inside_s);

    let s = String::from("texto");
    println!("{}", s);
    let mut greeting = String::from("Olá, ");
    greeting.push_str("Clayton");
    greeting += "!";
    println!("{}", greeting);

    // Copy
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);

    // Move
    let s1 = String::from("texto");
    let s2 = s1;
    // println!("{}, {}", s1, s2);

    // Clone
    let s1 = String::from("texto");
    let s2 = s1.clone();
    println!("{}, {}", s1, s2);

    // Borrow
    let s1 = String::from("texto");
    let s2 = &s1;
    println!("{}, {}", s1, s2);

    let s = String::from("texto");          // s entra em escopo
    take_ownership(s);                      // move o valor de s para dentro da função
    // println!("{} inside main()", s);     // s não é mais válido aqui

    let x = 5;                              // x entra em escopo
    make_copy(x);                           // copia o valor de x para dentro da função
    println!("{} inside main()", x);

    let s1 = return_value();                // move o valor retornado para s1
    let s2 = String::from("texto");         // s2 entra em escopo
    let s3 = take_and_return_value(s2);     // s2 é movida para para dentro da função
                                            // take_and_return_value, que também move o valor
                                            // retornado para s3
    println!("{}", s1);
    // println!("{}", s2);
    println!("{}", s3);

} // Fim do escopo de outside_s             // x e s saem de escopo. Como o valor de s foi movido,
                                            // nada acontece com s aqui
                                            // Aqui s1, s2 e s3 saem de escopo. s1 e s3 são
                                            // destruídas. Como s2 já foi movida nada demais acontece


fn take_ownership(s: String) { // s entra em escopo
    println!("{} inside takes_ownership()", s);
}   // Aqui s sai de escopo e o método 'drop' é chamado. A memória que guarda s é liberada

fn make_copy(x: i32) { // x entra em escopo
    println!("{} inside make_copy()", x);
} // Aqui x sai de escopo, mas como x é Copy, nada especial acontece

fn return_value() -> String {
    let s = String::from("texto"); // s entra em escopo
    s // s é movida para dentro da função que chamou return_value()
} // Aqui s sai de escopo, mas como o valor já foi movido na especial acontece

fn take_and_return_value(s: String) -> String {
    s // s é movida para dentro da função que chamou take_and_return_value()
} // Aqui s sai de escopo, mas como o valor já foi movido na especial acontece