fn main() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);
    
    // let string1 = "a string longa é longa".to_owned();
    // {
    //     let string2 = "xyz".to_owned();
    //     let resultado = maior(string1.as_str(), &string2);
    //     println!("A string mais longa é {}", resultado);
    // }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let resultado = maior(string1.as_str(), string2);
    println!("A string mais longa é {}", resultado);

    // let string1 = "a string longa é longa".to_owned();
    // let resultado;
    // {
    //     let string2 = "xyz".to_owned();
    //     resultado = maior(string1.as_str(), &string2);
    // }
    // println!("A string mais longa é {}", resultado);
}

fn maior<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
