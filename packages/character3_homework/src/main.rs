use std::io;

fn main() {
    let x:f64 = 36.0;
    let y = transter_tempature(x);
    println!("y is {}", y);

    let z = fibo(6);
    println!("z is {}", z);

    deal_music();
}


fn transter_tempature(x: f64) -> f64 {
    return x * 1.8 + 32.0;
}

fn fibo(x: u64) -> u64 {
    if x == 0  {
        return 1;
    }
    return x * fibo(x - 1);
}

fn deal_music() {
    let count =  [
        "First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth",
        "Tenth", "Eveth", "Twefth",
    ];

    println!("The twelve days of christmas");

    for day in count.iter() {
        println!("On the {} day of Christmas", day);
    }
}