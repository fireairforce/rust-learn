fn main() {
    let mut x: u32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let tup: (u32, f64, i32) = (500, 6.4, 1);

    let (a, b, c) = tup;
    println!("Val of b is {}", b);

    // 初始值为 0, 长度为 100 的数组
    let arr: [i32; 100] = [0; 100];
}

