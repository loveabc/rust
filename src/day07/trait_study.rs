use std::fmt::{Debug, Display};
use std::ops::Add;

fn test() {
    let twitter = Twitter {
        author: String::from("Musk"),
        title: String::from("Iran"),
        content: String::from("Iran attacked Israel 2 days ago"),
    };

    let summary = twitter.summarize();
    println!("{}", summary);

    let author = twitter.default_summary();
    println!("{}", author);

    notify(&twitter);
}

pub trait Summary {
    fn get_author(&self) -> String;
    fn summarize(&self) -> String;
    //以在特征中定义具有默认实现的方法，这样其它类型无需再实现该方法，或者也可以选择重载该方法
    fn default_summary(&self) -> String {
        format!("Read more from {}", self.get_author())
    }
}

struct Twitter {
    author: String,
    title: String,
    content: String,
}

impl Summary for Twitter {
    fn get_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.author, self.title, self.content)
    }
}

//使用Trait作为函数参数, 参数必须实现Summary Trait
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//两个参数都必须实现Summary Trait, 但是这两个参数的类型不一定完全一样, 比如A实现了Summary, B也实现了Summary, 那么参数可以是A和B, 虽然都实现了Summary, 但是他们不是同一个类型
pub fn notify1(item1: &impl Summary, item2: &impl Summary) {}

//两个参数类型必须相同, 而且必须实现Summary Trait
pub fn notify2<T: Summary>(item1: &T, item2: &T) {}

//notify3和notify4的约束完全相同, 参数都必须同时实现Summary和Display Trait
pub fn notify3(item: &(impl Summary + Display)) {}
pub fn notify4<T: Summary + Display>(item: &T) {}

//当特征约束变得很多时, 函数的签名将变得很复杂, 比如
//第一个参数需要同时实现Display和Clone Trait, 第二个参数需要同时实现Clone和Debug Trait
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    100
}

//上面的问题可以通过where解决
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug
{
    100
}

// 可以通过 impl Trait 来说明一个函数返回了一个类型，该类型实现了某个特征：
fn returns_summarize() -> impl Summary {
    Twitter {
        author: String::from("sunface"),
        content: String::from(
            "m1 max太厉害了，电脑再也不会卡",
        ),
        title: String::from("My Title"),
    }
}

struct Counter;
impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}