use std::cmp::Ord;

pub type Tree<T> = Option<Box<Node<T>>>;
// Implementation of a binary search tree
#[derive(Debug, Clone)]
pub struct Node<T> {
  pub data: T,
  pub right_child: Tree<T>,
  pub left_child: Tree<T>,
}

impl<T: Ord> Node<T> {
  /// Takes a node to add to the tree. It adds the node by value (copies it)
  pub fn add_node(&mut self, node: Node<T>) {
    self.add_new_node(node.data)
  }
  pub fn add_new_node(&mut self, node_data: T) {
    if self.data < node_data {
      match &mut self.right_child {
        Some(r_child) => r_child.add_new_node(node_data),
        None => self.right_child = Some(Box::new(Node::new(node_data))),
      }
    } else {
      match &mut self.left_child {
        Some(l_child) => l_child.add_new_node(node_data),
        None => self.left_child = Some(Box::new(Node::new(node_data))),
      }
    }
  }

  pub fn new(data: T) -> Self {
    Node {
      data,
      right_child: None,
      left_child: None,
    }
  }
}

mod test {
  #[test]
  fn can_add_nodes() {
    use super::*;

    let mut root = Node::new(4);
    let l_child = Node::new(2);
    let r_child = Node::new(5);
    let ll_grandchild = Node::new(1);
    let lr_grandchild = Node::new(3);

    root.add_node(l_child);
    root.add_node(r_child);
    root.add_node(lr_grandchild);
    root.add_node(ll_grandchild);

    assert_eq!(root.right_child.as_ref().unwrap().data, 5);
    assert_eq!(
      root
        .left_child
        .as_ref()
        .unwrap()
        .left_child
        .as_ref()
        .unwrap()
        .data,
      1
    );
    assert_eq!(
      root
        .left_child
        .as_ref()
        .unwrap()
        .right_child
        .as_ref()
        .unwrap()
        .data,
      3
    );
    assert_eq!(root.left_child.as_ref().unwrap().data, 2);
  }
}
