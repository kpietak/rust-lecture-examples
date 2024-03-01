/// main function: call from here a proper ex* method
fn main() {
    ex11();
}

/// a default lifecycle and scope of a variable
#[allow(dead_code)]
fn ex1() {
    // here s1 is not valid - not declared yet
    let s1 = String::from("Let's ..."); // s1 is created here

    // now you can perform some operations on s
}   // we are out of the scope, s is not valid (the value is dropped)

/// move with assignment
#[allow(dead_code)]
fn ex2() {
    let s1 = String::from("Hello Rust");
    let s2 = s1; // the value is moved from s1 to s2, s1 is no longer valid
                        // we can clone value here

    //println!("{}, {}", s1, s2); // error, s1 is not valid
}

/// move value by passing it as a function argument
#[allow(dead_code)]
fn ex3() {
    let s1 = String::from("Let's ...");
    process_text(s1);  // s1 is moved to the function

    // println!("{s1}"); // cannot use s1 here anymore
}

fn process_text(s : String) {
    println!("Processing text: {}", s);
}

/// get new value as a result from a function
#[allow(dead_code)]
fn ex4() {
    let s1 = String::from("Let's ...");
    let s2 = small(s1);  // s1 is moved to small function
                                // the value created in small function is moved to s2
    // println!("{s1}"); // cannot use here s1 anymore
}

fn small(s : String) -> String {
    s.to_lowercase() // creates a new string
}

/// reuse the value by returning a tuple
#[allow(dead_code)]
fn ex5() {
    let s1 = String::from("Let's ...");
    let (s1, s2) = small2(s1);

    println!("{s1}");
}
fn small2(s : String) -> (String, String) {
    (s.clone(), s.to_lowercase())
}

/// borrow a string value using reference
#[allow(dead_code)]
fn ex6() {
    let s1 = String::from("Let's learn Rust's ownership");
    let s2 = small3(&s1); // reference to s1, lents the value to small

    println!("{s1}, {s2}");
}

fn small3(s : &String) -> String { // borrows s value (the value isn't moved)
    s.to_lowercase() // returning a new value
} // s is out of scope here, but the value is not dropped (s doesn't own it)

/// change string value using mutable reference
#[allow(dead_code)]
fn ex7() {
    let mut s1 = String::from("Let's learn Rust's ownership."); // mutable variable
    change(&mut s1); // mutable reference to s1

    println!("{s1}");
}

fn change(s : &mut String) { // borrows mutable value
    s.push_str(" I'm so excited."); // the borrowed value is changed (in place)
} // s is out of scope here, but the value is not dropped (s doesn't own it)

/// illustration of reference rules
#[allow(dead_code)]
fn ex8() {
    let mut s1 = String::from("Let's learn Rust's ownership."); // mutable variable
    let s2 = &s1; // that's ok
    let s3 = &s1; // no problem here
    let s4 = &mut s1; // error, cannot have mutable reference when already exists non-mutable reference

    //println!("{s1}, {s2}, {s3}, {s4}");
}

/// illustration of dangling references
#[allow(dead_code)]
fn ex9() {
    // uncomment to check ex9
    //let s1 = getString(); // s1 is a dangling reference (to non-existing value)

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

// uncomment to check ex9
// fn getString() -> &String {
//     let s = String::from("I'm dangling text."); // creating new value here
//     &s // returns reference to s (s still owns the value, there is no move here)
// }  // s out of scope here so the value is dropped


/// how to use string slices
#[allow(dead_code)]
fn ex10() {
    let s = String::from("Hello Rust");
    let first = first_word(&s);

    println!("{first}");
}

#[allow(dead_code)]
fn first_word(s : &String) -> &str { // return slice of String

    for (i, c) in s.char_indices() {
        if c == ' ' {
            return &s[..i]; // return slice containing first word
        }
    }
    &s[..] // return slice containing whole text
}

/// int slice
fn ex11() {
    let a = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];

    let a1 = &a[0..5];
    let a2 = &a[..3];
    let a3 = &a[7..];

    println!("{:?}", a1);
    println!("{:?}", a2);
    println!("{:?}", a3);
}