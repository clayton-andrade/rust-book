extern crate traits;

use traits::{ Resumir, Tweet };

struct PrevisaoDoTempo {
    alta_temperatura: f64,
    baixa_temperatura: f64,
    chance_de_chuva: f64,
}

impl Resumir for PrevisaoDoTempo {
    fn resumo_autor(&self) -> String {
        format!("Autor não disponível.")
    }

    fn resumo(&self) -> String {
        format!("A alta será de {} °C, e a baixa de {} °C. A chance de precipitação é de {}%.",
                self.alta_temperatura, self.baixa_temperatura, self.chance_de_chuva)

    }
}

fn main() {
    let tweet = Tweet {
        nomeusuario: String::from("horse_ebooks"),
        conteudo: String::from("claro, como vocês provavelmente já sabem, pessoas"),
        resposta: false,
        retweet: false,
    };
    println!("1 novo tweet: {}", tweet.resumo());

    let previsao = PrevisaoDoTempo {
        alta_temperatura: 38.7,
        baixa_temperatura: 19.8,
        chance_de_chuva: 38.0,
    };
    traits::notificar(previsao);
}