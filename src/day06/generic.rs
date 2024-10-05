use std::ops::{Add, Mul};

fn test() {
    let v1 = 100;
    let v2 = 200;
    println!("v1 + v2 = {}", add(v1, v2));

    let point = Point { x: 5, y: 2 };
    println!("{}", point.area());
    let point = Point { x: 4.0f32, y: 3.0f32 };
    println!("{}", point.distance_from_origin());

    let arr = [1, 2, 3, 4, 5, 6];
    generic_arr(arr);
    let arr = [1.0, 2.0, 3.0];
    generic_arr(arr);

    let arr = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    generic_arr(arr);

    let arr = [5; 1024];
    let arr = [0u8; 512];
    generic_arr_length(arr);
}



fn generic_arr_length<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", size_of_val(&arr));
}

fn generic_arr<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
    println!("{:?}", size_of::<T>());
}

fn add<T: Add<Output=T>>(v1: T, v2: T) -> T {
    v1 + v2
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: Add<Output=T> + Mul<Output=T> + Copy,
{
    fn area(&self) -> T {
        self.x * self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}