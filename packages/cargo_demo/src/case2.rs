fn main() {
  let data = vec![1,2,3,4];
  // data 传给 data1 之后(data 指向的值移动到了 data1,因此 data 也不能访问了)
  let data1 = data;
  // data1 被传进去 sum 之后，之后就不能访问了
  println!("Sum of data1: {}", sum(data1));
  // 不能使用已经移动过的变量
  println!("data1: {:?}", data1); // err1
  println!("sum of data: {}", sum(data)); // err2
}

fn sum(data: Vec<u32>) -> u32 {
  data.iter().fold(0, |acc, x| acc + x)
}