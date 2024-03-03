
//基本数值类型
fn main() {
    println!("基本数值类型");
  
    let mut x: i32 = 100;
    println!("{}", x);
  
    x = 10000000;
    println!("{}", x);
  
    let v: u8 = 255;
    println!("{}", v);
    // v = v + 10;
    // println!("{}", v);
  
    let (vv, t) = v.overflowing_add(255);
    println!("{}", vv);
    println!("{}", t);
  
    let f1 = 3.14159;
    println!("{}", f1);
  
    let d1 = 3.14;
    println!("{}", d1);
  
    let xyz = (1, 2, 3);
    println!("{}", xyz.1);
  
    println!("----------------------------");
  
    // 加法
    let sum = 5 + 10;
    println!("{}", sum);
  
    // 减法
    let difference = 95.5 - 4.3;
    println!("{}", difference);
  
    // 乘法
    let product = 4 * 30;
    println!("{}", product);
  
    // 除法
    let quotient = 56.7 / 32.2;
    println!("{}", quotient);
  
    // 求余
    let remainder = 43 % 5;
    println!("{}", remainder);
  
    let arr = [2, 3, 4, 5, 6];
    println!("{}", arr[2]);
  
    println!("----------------------------");
  
    for i in 1..5 {
      println!("{}", i);
    }
    println!("----------------------------");
    for i in 1..=5 {
      println!("{}", i);
    }
    println!("----------------------------");
    for i in 'a'..'g' {
      println!("{}", i);
    }
    println!("----------------------------");
    for i in 'a'..='g' {
      println!("{}", i);
    }
    println!("----------------------------");
    println!("{}", 3.14_f32.round());
  }