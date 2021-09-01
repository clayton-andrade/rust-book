fn main() {
    let frase = String::from("Hello, world!");
    let palavra = primeira_palavra(&frase);
    println!("{}", palavra);

    let s = "texto longo";
    let s1 = primeira_palavra(&s);
    println!("{}", s1);

    // let mut s = String::from("texto longo");
    // let s1 = primeira_palavra(&s);
    // s.clear();
    // println!("{}, {}", s, s1);

    // slices de array
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}, {:?}", a, slice);
}

fn primeira_palavra(s: &str) -> &str {
    let chars = s.chars();
    for (i, elem) in chars.enumerate() {
        if elem == ' ' {
            return &s[..i];
        }
    }
    &s[..]
}
