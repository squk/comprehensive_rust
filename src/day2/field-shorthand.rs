// https://google.github.io/comprehensive-rust/structs/field-shorthand.html
// If you already have variables with the right names, then you can create the struct using a shorthand:
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    // This WONT compile:
    // fn newBad(nameBad: String, ageBad: u8) -> Person {
    //     Person { nameBad, ageBad }
    // }
}

fn main() {
    let peter = Person::new(String::from("Peter"), 27);
    println!("{peter:?}");
}
