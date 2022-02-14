use std::rc::Rc;
// a,b,c 共同指向堆上相同的数据，堆上的数据有三个共享的所有者
// 这段代码结束，c 先 drop, 引用计数 -> 2，然后 b、a drop 引用计数归0，堆上内存被释放
fn main() {
  let a = Rc::new(1);
  // 对 Rc 结构进行 clone(),不会将内部的数据复制，只会增加引用计数
  let b = a.clone();
  let c = a.clone();
}



// clone 内部实现
fn clone(&self) -> Rc<T> {
  // 增加引用计数
  self.inner().inc_strong();
  // 通过 self.ptr 生成一个新的 Rc 结构
  Self::from_inner(self.ptr)
}