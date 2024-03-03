
//所有权和借用

fn main() {
// 关于所有权的规则，首先请谨记以下规则：
// 1. Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
// 2. 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
// 3. 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)
  let s = String::from("hello, world");
  println!("{}", s);

  let s = take_ownership(s);//移动, 转移所有权, 函数又将所有权返回了
  
  //移动, 转移所有权
  let s2 = s;
  println!("{}", s2);


  //println!("{}", s); compile error

  let x = "HELLO";
  let y = x;
  println!("{}, {}", x, y);

  //将值传递给函数，一样会发生 移动 或者 复制

  //移动: 转移所有权, 之前的变量失效
  //引用与借用: 获取变量的引用，称之为借用

  println!("----------------------------");
  let a = String::from("borrow");
  let b = &a;
  println!("{}", a);
  println!("{}", *b);

  println!("----------------------------");
  let b1 = String::from("borrow test");
  borrow_ship(&b1);
  println!("{}", b1);
  // 正如变量默认不可变一样，引用指向的值默认也是不可变的
  // change(&b1);

  println!("----------------------------");
  //可变引用
  let mut b2 = String::from("borrow mut");
  let mutref = &mut b2;
  change_mut(mutref);
  
  println!("{}", b2);

  //重要 重要 重要 -> 可变引用同时只能存在一个
  let z = &mut b2;
  // let zz = &mut b2; //如果下面使用了z, 这儿报错, 因为有2个可变引用
  //println!("{}", z); 这儿使用了z, 上面一行会报错

  println!("----------------------------");
  // 可变引用与不可变引用不能同时存在
  let mut s = String::from("hello");

let r1 = &s; // 没问题
let r2 = &s; // 没问题
// let r3 = &mut s; // 大问题
// println!("{}, {}, and {}", r1, r2, r3);

// 注意，引用的作用域 s 从创建开始，一直持续到它最后一次使用的地方，这个跟变量的作用域有所不同，变量的作用域从创建持续到某一个花括号 }

// 借用规则总结
// 总的来说，借用规则如下：
// 同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
// 引用必须总是有效的


}


fn take_ownership(s: String) -> String {
  println!("{}", s);
  s
}

fn borrow_ship(s: &String) {
  println!("{}", s.len());
}

fn change(s: &String) {
  // s.push_str("change"); 这儿会报错, 因为引用指向的值默认也是不可变的
}

fn change_mut(s: &mut String) {
  s.push_str("change");
}
