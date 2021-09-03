#![allow(unused_variables)]

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn build_user(username: String, email: String) -> Self {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }
}

struct RGBColor(u8, u8, u8);

struct Point(f32, f32);

fn main() {
    let user1 = User {
        username: String::from("clayton"),
        email: String::from("clayton@email.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("{:?}", user1);

    let mut user2 = User {
        username: String::from("clayton"),
        email: String::from("heloiza@email.com"),
        ..user1
    };
    println!("{:?}", user2);
    user2.username = String::from("heloiza");
    println!("{:?}", user2);

    let user3 = User::build_user(String::from("nina"), String::from("nina@email.com"));
    println!("{:#?}", user3);

    let black = RGBColor(0, 0, 0);
    let origin = Point(0.0, 0.0);

    
}
