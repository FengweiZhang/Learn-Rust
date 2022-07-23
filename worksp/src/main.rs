
fn main() {
    let mut s = String::from("12345是");
    let word = &s[0..3];

    s.clear();
    println!("{}", word);
}

