#![allow(dead_code)]

enum VersaoIP {
    V4,
    V6,
}

// #[derive(Debug)]
// enum EnderecoIP {
//     V4(String),
//     V6(String),
// }

#[derive(Debug)]
enum EnderecoIP {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Mensagem {
    Sair,
    Mover { x: i32, y: i32 },
    Escrever(String),
    MudarCor(u8, u8, u8),
}

impl Mensagem {
    fn invocar(&self) {
        match self {
            Mensagem::Sair => println!("Saindo..."),
            Mensagem::Mover { x, y } => println!("Movendo para a posição (x: {}, y: {})", x, y),
            Mensagem::Escrever(s) => println!("{}", s),
            Mensagem::MudarCor(r, g, b) => println!("Mudando a cor para RGB({}, {}, {})", r, g, b),
        }
    }
}

// struct EnderecoIP {
//     versao: VersaoIP,
//     endereco: String,
// }

fn main() {
    // let loopback_v4 = EnderecoIP {
    //     versao: VersaoIP::V4,
    //     endereco: String::from("127.0.0.1"),
    // };
    // let loopback_v6 = EnderecoIP {
    //     versao: VersaoIP::V6,
    //     endereco: String::from("::1"),
    // };
    let loopback_v4 = EnderecoIP::V4(127, 0, 0, 1);
    let loopback_v6 = EnderecoIP::V6(String::from("::1"));
    println!("{:?}", loopback_v4);
    println!("{:?}", loopback_v6);

    let msg1 = Mensagem::Mover { x: 25, y: 32 };
    let msg2 = Mensagem::Sair;
    let msg3 = Mensagem::MudarCor(123, 29, 78);
    let msg4 = Mensagem::Escrever("Escrevendo na tela...".into());
    msg1.invocar();
    msg2.invocar();
    msg3.invocar();
    msg4.invocar();
}