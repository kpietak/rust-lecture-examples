fn main() {
    ex1();
}

fn ex1() {
    let a = [1, 2, 3, 4, 5];
    let b = [10, 11, 12, 13, 14, 15];

    let len = len_longer_array(&a, &b);
    println!("{len}");

    let longer = longer_array(&a, &b);
    println!("{:?}", longer);
}

fn len_longer_array(a : &[i32], b : &[i32]) -> usize {
    if a.len() > b.len() {
        a.len()
    } else {
        b.len()
    }
}

fn longer_array<'a>(a : &'a[i32], b : &'a[i32]) -> &'a[i32] {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}


fn ex2() {
    let text = String::from("Introduction to a long text. The rest of long text with many sentences.");

    let intro = text.split('.').next().expect("Could not find a first sentence.");

    let i = Introduction { intro};

    i.print();
}
struct Introduction<'a> {
    intro : &'a str
}

impl<'a> Introduction<'a> {
    fn print(&self) {
        println!("{}", self.intro);
    }
}


fn get_sample_text() -> &'static str {
    "Just a sample text"
}