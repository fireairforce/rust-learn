// linked-list
use std::rc::{ Rc, Weak };
use std::cell::RefCell;
use std::fmt::Display;

struct Node<T> {
  pub data: T,
  pub prev: Option<Weak<RefCell<Node<T>>>>,
  pub next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
  pub fn new(data: T) -> Self {
    Self {
      data,
      prev: None,
      next: None
    }
  }

  pub fn append(node: &mut Rc<RefCell<Node<T>>>, data: T) -> Option<Rc<RefCell<Node<T>>>> {
    let is_last = node.borrow().next.is_none();
    if is_last {
      let mut new_node = Node::new(data);
      new_node.prev = Some(Rc::downgrade(&node));
      let rc = Rc::new(RefCell::new(new_node));
      node.borrow_mut().next = Some(rc.clone());
      Some(rc)
    } else {
      if let Some(ref mut next) = node.borrow_mut().next {
        Self::append(next, data)
      } else { None }
    }
  }
}

struct List<T> {
  first: Option<Rc<RefCell<Node<T>>>>,
  last: Option<Rc<RefCell<Node<T>>>>,
}

impl <T> List<T> {
  pub fn new() -> Self {
    Self { first: None, last: None }
  }

  pub fn append(&mut  self, data: T) {
    if let Some(ref mut next) = self.first {
      self.last = Node::append(next, data);
    } else {
      let f = Rc::new(RefCell::new(Node::new(data)));
      self.first = Some(f.clone());
      self.last = Some(f);
    }
  }
}

impl<T: Display> Display for List<T> {
  fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    write!(w, "[")?;
    let mut node = self.first.clone();
    while let Some(n) = node {
      write!(w, "{}", n.borrow().data)?;
      node = n.borrow().next.clone();
      if node.is_some() {
        write!(w, ", ")?;
      }
    }
    write!(w, "]")
  }
}

fn main() {
  let mut list = List::new();
  println!("{}", list);
  for i in 0..5 {
    list.append(i);
  }
  println!("{}", list);
}