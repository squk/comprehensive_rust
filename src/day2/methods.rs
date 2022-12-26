#[derive(Debug)]
struct Person {
    name : String,
    age: u8,
}

impl Person     {
    fn say_hello(&self) {
        println!("Hello my name is {}", self.name)
    }
}

fn main() {
    let christian = Person {
        name : String::from("Christian"),
        age: 25,
    };
    christian.say_hello()
}
