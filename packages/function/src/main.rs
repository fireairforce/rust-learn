fn main() {
    println!("test");
    print_hello_world();
    output_num(2, 3);
    test_case();
    let z: i32 = five();
    println!("val z is {}", z);
}

fn print_hello_world() {
    println!("hello~");
}

fn output_num(x: u32, y: u32) {
    println!("x is {}", x);
    println!("y is {}", y);
}

fn test_case() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("val of y is {}", y);
}

// 带返回值

fn five() -> i32 {
    -5
}