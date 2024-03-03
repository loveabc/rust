
//函数


fn main() {
    println!("函数");
    another_function(5, 6.1);
  
    println!("----------------------------");
    let v = plus_or_minus(2);
    println!("{}", v);
  
    dead_end();
  }
  
  fn another_function(x: i32, y: f32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
  }
  
  fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
      return x - 5;
    }
  
    x + 5
  }
  
  fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
  }
  