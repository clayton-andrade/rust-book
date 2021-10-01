mod lib;

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count > 5 {
            return None;
        }
        Some(self.count)
    }
}

fn main() {
    let mut v1 = vec![1, 2, 3, 4, 5];

    let v1_iter = v1.iter();
    for item in v1_iter {
        println!("{}", item);
    }

    println!();

    let mut v1_iter = v1.iter();
    while let Some(v) = v1_iter.next() {
        println!("{}", v);
    }

    let v2 = vec![1, 2, 3];

    let mut v2_iter = v2.iter();
    assert_eq!(v2_iter.next(), Some(&1));
    assert_eq!(v2_iter.next(), Some(&2));
    assert_eq!(v2_iter.next(), Some(&3));
    assert_eq!(v2_iter.next(), None);

    let mut v2_iter = v2.into_iter();
    assert_eq!(v2_iter.next(), Some(1));
    assert_eq!(v2_iter.next(), Some(2));
    assert_eq!(v2_iter.next(), Some(3));
    assert_eq!(v2_iter.next(), None);

    v1 = v1.iter().map(|x| x + 1).collect::<Vec<i32>>();
    println!("v1 = {:?}", v1);

    let shoes = vec![
            lib::Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            lib::Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            lib::Shoe {
                size: 10,
                style: String::from("boot"),
            },
    ];

    let in_my_size = lib::shoes_in_size(shoes, 10);
    println!("Shoes: {:?}", in_my_size);

    println!();

    let my_iter = Counter::new();
    for i in my_iter {
        println!("{}", i);
    }
}
