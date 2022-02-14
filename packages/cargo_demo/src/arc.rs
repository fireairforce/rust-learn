// 不跨线程访问，使用 Rc 以及 RefCell
// 要跨线程，使用 Arc 以及 Mutex 或者 RwLock

fn main() {
  let arr = vec![1];

  std::thread::spawn(move || {
    println!("{:?}", arr);
  });
}