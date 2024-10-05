fn test_array() {
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr2);

    let arr3 = [3; 5];
    println!("{:?}", arr3);

    println!("{:?}", arr[0]);
    println!("{:?}", arr2[1]);
    println!("{:?}", arr3[2]);

    // println!("please input arr index: ");
    // let mut index = String::new();
    // std::io::stdin().read_line(&mut index).expect("Failed to read line");
    // let index: usize = index.trim().parse().expect("Index entered was not a number");
    // println!("index is {}", index);
    // println!("{:?}", arr[index]);

    //string array
    let string_arr = ["hello", "world"];
    println!("{:?}", string_arr);

    let string_arr2 = [String::from("hello"), String::from("world")];
    println!("{:?}", string_arr2);

    let string_arr3: [String; 3] = std::array::from_fn(|_i| { String::from("hello") });
    println!("{:?}", string_arr3);

    let array: [i32; 10] = std::array::from_fn(|i| i as i32 * 2);
    println!("{:?}", array);

    let new_arr_slice = &array[1..6];
    println!("{:?}", new_arr_slice);

    let two_array = [[5; 3]; 2];
    println!("{:?}", two_array);

    for arr in two_array.iter() {
        for i in arr.iter() {
            println!("{:?}", i);
        }
    }

    let mut sum = 0;
    for i in 0..101 {
        sum += i;
    }
    println!("{}", sum);
}