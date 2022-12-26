// If the field names are unimportant, you can use a tuple struct:
struct Point(i32, i32);

// This is often used for single-field wrappers (called newtypes).
// Single-field wrappers are similar to type-aliases from Golang.
#[derive(Debug)]
struct Newtons(f64);

fn compute_thruster_force() -> Newtons {
    todo!("Ask a rocket scientist at NASA")
}

fn main() {
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);

    let force = compute_thruster_force();
    println!("{:?}", force)
}
