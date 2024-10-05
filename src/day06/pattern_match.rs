use std::fmt::Alignment;

fn main() {
    match_pattern(Direction::EAST);
    match_pattern(Direction::SOUTH);
    match_pattern(Direction::WEST);
    match_pattern(Direction::NORTH);
    match_pattern(Direction::OTHER);
    println!("--------------------------------------");
    let mut coin_number = match_coin(Coin::Penny);
    println!("coin number is {:?}", coin_number);
    coin_number = match_coin(Coin::Nickel);
    println!("coin number is {:?}", coin_number);
    coin_number = match_coin(Coin::Dime);
    println!("coin number is {:?}", coin_number);
    coin_number = match_coin(Coin::Quarter(UsState::Alabama));
    println!("coin number is {:?}", coin_number);
    coin_number = match_coin(Coin::Quarter(UsState::Alaska));
    println!("coin number is {:?}", coin_number);

    match_one(1);
    match_one(2);

    match_x(vec![Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Penny, Coin::Penny,]);

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.pop();
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v1 = Some(5);
    let v2 = 500;
    match v1 {
        Some(100) => println!("The value of v1 is {}", 100),
        Some(x) => println!("The value of v2 is {}", x),
        _ => {}
    };
}

//matches!宏
fn match_x(coins: Vec<Coin>) {
    let only_penny: Vec<&Coin> = coins.iter().filter(|coin| matches!(coin, Coin::Penny)).collect();
    println!("{:?}", only_penny);
}

// match target {
// 模式1 => 表达式1,
// 模式2 => {
// 语句1;
// 语句2;
// 表达式2
// },
// _ => 表达式3
// }

enum Direction {
    EAST,
    SOUTH,
    WEST,
    NORTH,
    OTHER,
}

fn match_pattern(v: Direction) {
    match v {
        Direction::EAST => println!("East"),
        Direction::NORTH => println!("North"),
        Direction::WEST => println!("West"),
        Direction::SOUTH => println!("South"),
        _ => println!("Other")
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn match_coin(coin: Coin) -> i8 {
    let coin_number = match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        },
    };
    coin_number
}

fn match_one(v: i32) {
    if let 1 = v {
        println!("match_one");
    }
}


