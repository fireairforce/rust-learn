use std::rc::Rc;

#[derive(Debug)]
struct Node {
  id: usize,
  downstream: Option<Rc<Node>>,
}

impl Node {
  // 创建新的 Node
  pub fn new(id: usize) -> Self {
    Self {
      id,
      downstream: None,
    }
  }

  // 设置 Node 的 downstream
  pub fn update_downstream(&mut self, downstream: Rc<Node>) {
    self.downstream = Some(downstream);
  }

  // clone 一份 Node 里的 downstream
  pub fn get_downstream(&self) -> Option<Rc<Node>> {
    self.downstream.as_ref().map(|v| v.clone())
  }
}

fn main() {
  let mut node1 = Node::new(1);
  let mut node2 = Node::new(2);
  let mut node3 = Node::new(3);
  let node4 = Node::new(4);
  node3.update_downstream(Rc::new(node4));

  node1.update_downstream(Rc::new(node3));
  node2.update_downstream(node1.get_downstream().unwrap());
  println!("node1: {:?}, node2: {:?}", node1, node2);
}