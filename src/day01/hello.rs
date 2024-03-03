fn main() {

    let v = 10;//v是 不可变的
    //v = 20; //如果在这儿修改v, 有语法错误, 因为修改了不可变的值
    println!("{}", v);

    let mut s = 10;//s是可变的
    println!("{}", s);
    s = 20;//因为s是可变的, 所有这儿可以修改s的值
    println!("{}", s);
    println!("----------------------------");

    let _k = 20;
    let mut _z = 10;

    //变量解构
    let (a, mut b):(bool, bool) = (true, true);
    println!("{}", a);
    println!("{}", b);
    b = false;
    println!("{}", b);
    println!("----------------------------");

    //解构式赋值
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    (c, .., d, _) = (3, 4, 5, 6, 7, 8, 9, 10);
    Struct {e, ..} = Struct {e: 100};

    println!("{}, {}, {}, {}, {}", a, b, c, d, e);

    // 不可变变量: 允许另外一个不可变变量或者可变变量使用同一个变量名字, 这样做会把之前的不可变变量遮蔽掉(可变变量也允许这样操作)
    //常量: 常量的名字不允许被其他常量, 可变变量以及不可变变量再次使用
    let _mu = 10;
    let _mu = 20;
    let mut _mu = 30;
    let mut _mu = 40;
    let _mu: i32 = 50;

    const _MU: i32 = 10;
    //const _MU: i32 = 20;//compile error

    println!("----------------------------");
    let x = 5;
    let x = x + 1;
    {
      let x = x * 2;
      println!("{}", x);
    }
    println!("{}", x);

    println!("----------------------------");
    let mut xxx = 5;
    xxx = xxx + 1;
    {
      xxx = xxx * 2;
      println!("{}", xxx);
    }
    println!("{}", xxx);

    println!("----------------------------");
    let spaces = "      ";
    let spaces = spaces.len();
    println!("{}", spaces);

    let mut x: i32 = 100;
    println!("{}", x);

    x = 10000000;
    println!("{}", x);


  }

  struct Struct {
    e: i32
  }