static mut COUNTER: u64 = 0;

fn main() {
  // 需要 unsafe 才能在多线程下修改全局变量的值
  COUNTER += 1;
}