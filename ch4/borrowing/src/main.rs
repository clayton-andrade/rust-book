#![allow(unused_variables, unused_mut)]

fn main() {
    let s = "texto".to_string();
    let tamanho = calcula_tamanho(&s);
    println!("O tamanho de {} é {}", s, tamanho);

    let mut s = String::from("texto");
    println!("{}", s);
    modifica(&mut s);
    println!("{}", s);

    let r1 = &mut s;
    // let r2 = &mut s;
    println!("{}", r1);
    // println!("{}", r2);

    let mut s = String::from("texto");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
    println!("{}", r1);
    println!("{}", r2);

    {
        let mut s = String::from("texto");
        let s1 = &s;
        // let s2 = &mut s;
        // println!("{}, {}", s1, s2);
    }

    {
        let mut s = String::from("texto");
        let s1 = &s;
        let s2 = &s;
        // let s3 = &mut s;
        println!("{}, {}", s1, s2);
        // println!("{}, {}, {}", s1, s2, s3);
    }

    {
        let mut s = String::from("texto");
        let s1 = &s;
        let s2 = &s;
        println!("{}, {}", s1, s2);
        let s3 = &mut s;
        println!("{}", s3);
    }

    // let referencia_para_o_nada = soltar();
}

fn calcula_tamanho(s: &String) -> usize { // s é uma referencia para uma String
    s.len()
} // aqui s sai de escopo, mas como ela não possui o valor a que se refere, nada acontece

fn modifica(s: &mut String) {
    s.push_str(" longo");
}

// fn soltar() -> &String { // soltar() retorna uma referencia para uma String
//     let s = String::from("texto"); // s é uma nova String
//     &s // retornamos uma referencia a uma String
// } // Aqui s sai de escopo e é destruída. Sua memória é devilvida. Perigo!