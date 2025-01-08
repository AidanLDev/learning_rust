#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self, hello: &str) -> u32 {
        println!("Self is {:#?}", self);
        println!("Hello from the param {hello}");
        self.width * self.height
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("Aidan"),
        email: String::from("aidan@mail.com"),
        sign_in_count: 69,
    };

    println!("user1 is {:#?}", user1);

    let rectangle1 = Rectangle {
        height: 120,
        width: 50,
    };

    let hello = "Hello";

    println!("Using the method to workout the area: {}",rectangle1.area(&hello));

    println!("The area of rectangle1 is {}.", area(&rectangle1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
