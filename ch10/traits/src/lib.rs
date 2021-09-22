pub trait Resumir {
    fn resumo_autor(&self) -> String;

    fn resumo(&self) -> String {
        format!("(Leia mais de {}...)", self.resumo_autor())
    }
}

pub struct ArtigoDeNoticia {
    pub titulo: String,
    pub local: String,
    pub autor: String,
    pub conteudo: String,
}

impl Resumir for ArtigoDeNoticia {
    fn resumo_autor(&self) -> String {
        format!("{}", self.autor)
    }

    fn resumo(&self) -> String {
        format!("{}, by {} ({})", self.titulo, self.autor, self.local)
    }
}

pub struct Tweet {
    pub nomeusuario: String,
    pub conteudo: String,
    pub resposta: bool,
    pub retweet: bool,
}

impl Resumir for Tweet {
    fn resumo_autor(&self) -> String {
        format!("@{}", self.nomeusuario)
    }
}

pub fn notificar<T: Resumir>(item: T) {
    println!("Notícias de última hora! {}", item.resumo());
}