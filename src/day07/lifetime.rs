
fn test() {
    let r = 1;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    println!("r: {}", r);

    let x = String::from("xxx");
    let y = "yyyyy";
    let large = largest(x.as_str(), y);
    println!("large: {}", large);

    println!("x: {}", x.as_str());
    println!("x: {}", y.to_string());
}


fn largest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Person<'a> {
    name: &'a str,
}