// https://google.github.io/comprehensive-rust/ownership/copy-clone.html
#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn main() {
    // Copied by default
        let x = 42;
    let y = x;
    println!("x: {x}");
    println!("y: {y}");

    // implements the Copy trait
    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}
