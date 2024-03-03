
//语句和表达式

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1;
    let y = y +1;
    x + y
    //OR return x + y;
  }
  fn main() {
    println!("语句和表达式");
  
    let v = add_with_extra(10, 20);
    println!("{}", v);
  
    println!("----------------------------");
  
    let y = {
      let x = 10;
      x + 10
    };
    println!("{}", y);
  
    println!("----------------------------");
  
    let x = 101;
    let v = if x % 2 == 1 {"odd"} else {"even"};
    println!("{}", v);
  }
  