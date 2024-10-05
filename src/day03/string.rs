fn test_string() {
    let mut s = String::from("hello world");
    println!("{}", s);
    s.push_str(" world");
    println!("{}", s);
}

//enum
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None
    }
}