use std::cell::RefCell;
use std::rc::Rc;

pub type DoubleLinkedTree<'a, T> = Option<Rc<RefCell<DoubleLinkedNode<'a, T>>>>;
// Implementation of a binary search tree
#[derive(Debug, Clone)]
pub struct DoubleLinkedNode<'a, T> {
  pub data: T,
  pub right_child: DoubleLinkedTree<'a, T>,
  pub left_child: DoubleLinkedTree<'a, T>,
  pub parent: Option<*const DoubleLinkedNode<'a, T>>,
}

impl<'a, T: Ord> DoubleLinkedNode<'a, T> {
  pub fn add_new_node(&mut self, node_data: T) {
    if self.data < node_data {
      match &mut self.right_child {
        Some(r_child) => r_child.borrow_mut().add_new_node(node_data),
        None => {
          let r_child = Rc::new(RefCell::new(DoubleLinkedNode::new(node_data)));
          r_child.borrow_mut().parent = Some(self);
          self.right_child = Some(r_child);
        }
      }
    } else {
      match &mut self.left_child {
        Some(l_child) => l_child.borrow_mut().add_new_node(node_data),
        None => {
          let l_child = Rc::new(RefCell::new(DoubleLinkedNode::new(node_data)));
          l_child.borrow_mut().parent = Some(self);
          self.left_child = Some(l_child);
        }
      }
    }
  }

  pub fn new(data: T) -> Self {
    DoubleLinkedNode {
      data,
      right_child: None,
      left_child: None,
      parent: None,
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn main_test() {
    let mut root = DoubleLinkedNode::new(0);
    root.add_new_node(5);
    root.add_new_node(4);
    root.add_new_node(3);
    root.add_new_node(-1);

    let r_child = &root.right_child;
    let r_child = r_child.as_ref().unwrap().borrow();
    let rl_child = r_child.left_child.as_ref().unwrap().borrow();

    unsafe {
      assert_eq!(r_child.data, (*rl_child.parent.unwrap()).data);
    }
    assert_eq!(5, r_child.data);
    assert_eq!(4, rl_child.data);
  }
}
