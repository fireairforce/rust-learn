use std::io;

fn main() {
    println!("guess");
    println!("Input: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Fail read");

    println!("You guess is {}", guess);
}
