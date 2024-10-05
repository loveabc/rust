fn test() {
    let mut initial_number = 50;

    //闭包可以捕获调用者作用域中的值, 这儿的闭包就是initial_number被捕获

    // 所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
    // 没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
    // 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
    let add = |a, b| a + b + initial_number;
    let minus = |a: i32, b: i32| a - b + initial_number;
    let mul = |a: i32, b: i32| a * b + initial_number;
    let divide = |a: i32, b: i32| a / b + initial_number;

    println!("{}", add(100, 20));
    println!("{}", minus(100, 20));
    println!("{}", mul(100, 20));
    println!("{}", divide(100, 20));
    println!("{}", divide(100, 20));

    let f = factory();
    println!("{}", f(10));

    println!("{:?}", test_struct());


}

fn factory() -> Box<dyn Fn(i32) -> i32> {
    let initial_count = 200;

    if initial_count == 100 {
        Box::new(move |x| x)
    } else {
        Box::new(move |x| x + initial_count)
    }
}

#[derive(Debug)]
struct Person {
    name: String,
}

//返回值Box里面没有dyn
fn test_struct() -> Box<Person> {
    Box::new(Person {
        name: "Musk".to_string(),
    })
}

trait Animal {

}

struct Dog;
impl Animal for Dog {

}

//使用Box的方式实质上是在传递一个指向堆上数据的指针，而不是直接传递具体的数据本身
//对于trait, 因为具体类型只有在runtime才能确定, 所以需要添加dyn, 对于struct, 编译的时候就能确定类型, 所以不需要dyn, 也不能加dyn

//返回值Box里面有dyn
fn test_trait() -> Box<dyn Animal> {
    Box::new(Dog {

    })
}

fn callback<F>(calculate: F, v1: i32, v2: i32) -> i32
where
    F: FnOnce(i32, i32) -> i32,
{
    calculate(v1, v2)
}
