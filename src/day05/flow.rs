// control flow, if, while/for/loop
fn check(flag: bool, first: i32, second: i32) {
    let v = if flag {
        first
    } else {
        second
    };
    println!("{}", v);

    let arr = [1,2,3,4,5,6,7,8,9,10];
    for (index, value) in arr.iter().enumerate() {
        println!("{} : {}", index, value);
    }

    let arr2: [i32; 6] = std::array::from_fn(|i| i as i32 * 2);
    for item in arr2 {
        if item == 8 {
            break;
        }
        println!("{}", item);
    }
}