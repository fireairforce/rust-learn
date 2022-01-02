// a 、b 这里都是可变引用，这里不用付出所有权
fn next_caclue(a: &mut i32, b: &mut i32) {
  let c = *a + *b;
  *a = *b;
  *b = c;
}

fn fib_for(_u: u8) {
  let (mut a, mut b) = (1, 1);

  for _i in 2.._u {
    next_caclue(&mut a, &mut b);
    println!("next val is {}", b);
  }
}

fn main () {
  let _n = 10;
  fib_for(10);
}