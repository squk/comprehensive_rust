// https://google.github.io/comprehensive-rust/ownership/moves-function-calls.html
fn say_hello(name: String) {
    println!("Hello {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(name);
    // say_hello(name);
}
