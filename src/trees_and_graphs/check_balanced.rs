use crate::trees_and_graphs::binary_search_tree::Node;
use std::cmp;

// Solution to Exercise 4.4

type Tree<T> = Option<Box<Node<T>>>;

/// Given a binary tree, determine whether it is balanced or not.
/// Return true if the node is balanced, false otherwise
///
/// # Example
/// ```
/// use ctci::trees_and_graphs::binary_search_tree::Node;
/// use ctci::trees_and_graphs::check_balanced::check_balanced;
///
/// let mut root = Node::new(4);
/// let l_child = Node::new(2);
///
/// root.add_node(l_child);
///
/// assert!(check_balanced(&Some(Box::new(root.clone()))));
/// ```
pub fn check_balanced<T>(root: &Tree<T>) -> bool {
  check_balanced_helper(root).is_some()
}
// This function takes in a node and returns its height if balanced, or None if unbalanced
fn check_balanced_helper<T>(root: &Tree<T>) -> Option<usize> {
  match root {
    None => Some(0),
    Some(node) => {
      let l_height = check_balanced_helper(&node.left_child)?;
      let r_height = check_balanced_helper(&node.right_child)?;
      if abs_diff(l_height, r_height) <= 1 {
        // Children are of similar height
        Some(cmp::max(l_height, r_height) + 1)
      } else {
        // One child is much taller than the other
        None
      }
    }
  }
}
fn abs_diff(a: usize, b: usize) -> usize {
  (a as isize - b as isize).unsigned_abs()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn main_test() {
    let mut root = Node::new(4);
    let l_child = Node::new(2);
    let r_child = Node::new(5);
    let ll_grandchild = Node::new(1);
    let lll_ggrandchild = Node::new(0);
    let lr_grandchild = Node::new(3);
    root.add_node(l_child);
    root.add_node(r_child);
    root.add_node(ll_grandchild);
    assert!(check_balanced(&Some(Box::new(root.clone()))));
    root.add_node(lll_ggrandchild);
    assert!(!check_balanced(&Some(Box::new(root.clone()))));
    root.add_node(lr_grandchild);

    assert!(!check_balanced(&Some(Box::new(root.clone()))));
  }
}
