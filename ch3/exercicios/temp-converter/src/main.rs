use std::env::args;

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() < 3 {
        println!("Usage: ./temp-converter temperature scale");
    } else {
        let temp = &args[1];
        let scale = &args[2];
        if let Ok(x) = temp.parse::<f64>() {
            let converted_temp;
            match scale.as_str() {
                "C" => {
                    converted_temp = 1.8 * x + 32.0;
                    println!("{} °{} -> {} °F", temp, scale, converted_temp);
                },
                "F" => {
                    converted_temp = (x - 32.0) / 1.8;
                    println!("{} °{} -> {} °C", temp, scale, converted_temp);
                },
                _ => println!("Invalid scale."),
            }
        }
        else {
            println!("Invalid temperature.");
        }
    }
}
