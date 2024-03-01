/// a basic illustration of Rust ownership
fn main() {
    let s1 = String::from("Hello Rust");
    let s2 = s1; // the value is moved from s1 to s2, s1 is no longer valid

    // println!("{}, {}", s1, s2); // error, s1 is not valid
}