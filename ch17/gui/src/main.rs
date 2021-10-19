#![allow(dead_code)]

use gui::Draw;
use gui::{Screen, Button};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox");
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 10,
                height: 10,
                options: vec![],
            }),
            Box::new(Button {
                width: 8,
                height: 4,
                label: String::from("Clique aqui"),
            }),
        ],
    };
    screen.run();
}