use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue".to_owned(), 10);
    scores.insert("Yellow".to_owned(), 50);
    let blue = scores["Blue"];
    println!("{}", blue);

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores = teams.iter().zip(initial_scores.iter()).collect::<HashMap<_, _>>();
    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{}", field_name);
    // println!("{}", field_value);

    let key = String::from("Yellow");
    let value = scores.get(&key);
    println!("{}-{}", key, value.unwrap());

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!();
    let mut scores = HashMap::new();
    scores.insert("Blue".to_owned(), 10);
    scores.insert("Yellow".to_owned(), 50);
    println!("{:?}", scores);
    scores.entry("Red".to_owned()).or_insert(30);
    println!("{:?}", scores);
    scores.entry("Blue".to_owned()).or_insert(100);
    println!("{:?}", scores);

    let s = "É comum querer verificar se uma determinada chave tem um valor e, se não tiver, inserir um valor para ela";
    let mut letters = HashMap::new();
    for c in s.chars() {
        if !c.is_alphabetic() {
            continue;
        }
        let count = letters.entry(c).or_insert(0);
        *count += 1;
    }
    for (letter, count) in &letters {
        println!("{}: {}", letter, count);
    }
}
