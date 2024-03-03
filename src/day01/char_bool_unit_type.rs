
//字符、布尔、单元类型
fn main() {
    println!("字符、布尔、单元类型");
  
    //在 Rust 语言中这些都是字符，Rust 的字符不仅仅是 ASCII，所有的 Unicode 值都可以作为 Rust 字符，包括单个的中文、日文、韩文、emoji 表情符号等等，都是合法的字符类型
    let c = 'z';
    println!("{}", c);
    let z = 'ℤ';
    println!("{}", z);
    let g = '国';
    println!("{}", g);
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);
  
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));
  
    println!("----------------------------");
    let t = true;
    println!("{}", t);
  }