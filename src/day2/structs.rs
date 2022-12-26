// https://google.github.io/comprehensive-rust/structs.html

struct Person {
    name: String,
    age: u8,
}

fn main() {
    let peter = Person {
        name: String::from("Christian"),
        age: 25,
    };

    println!("{} is {} years old", peter.name, peter.age);
}
