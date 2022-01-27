fn main() {
    let num = 3;

    if num < 5 {
        println!("small");
    } else {
        println!("large");
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("res is {}", result);

    while_case();
    for_case();
}

fn while_case() {
  let mut number = 3;

  while number != 0 {
      println!("{}!", number);

      number = number - 1;
  }
  println!("end");
}

fn for_case() {
    let a = [10,20,30,40,50];

    for element in a.iter() {
        println!("val is {}: ", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIETOFF!")
}