struct Par<T> {
    x: T,
    y: T,
}

impl<T> Par<T> {
    fn new(x: T, y: T) -> Self {
        Par { x, y }
    }
}

impl<T> Par<T>
where
    T: std::fmt::Display + PartialOrd,
{
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("O maior membro é {}", self.x);
        } else if self.x < self.y {
            println!("O maior membro é {}", self.y);
        } else {
            println!("Os membros {} e {} são iguais", self.x, self.y);
        }
    }
}

fn main() {
    let lista_numero = vec![34, 50, 25, 100, 65];
    let resultado = maior(&lista_numero);
    println!("O maior número é {}", resultado);

    let lista_char = vec!['c', 'h', 'x', 'm', 'q', 'k', 'j', 'i'];
    let resultado = maior(&lista_char);
    println!("O maior caractere é {}", resultado);

    let par = Par::new(10, 13);
    par.cmp_display();
}

// fn maior(list: &[i32]) -> i32 {
//     let mut maior = list[0];
//     for &numero in list.iter() {
//         if numero > maior {
//             maior = numero;
//         }
//     }
//     maior
// }

fn maior<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut maior = &list[0];
    for item in list.iter() {
        if item > maior {
            maior = item;
        }
    }
    maior
}
