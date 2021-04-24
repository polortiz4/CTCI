use crate::trees_and_graphs::binary_search_tree::Node;
use std::cmp;

// Solution to Exercise 4.4

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
/// assert!(check_balanced(&root));
/// ```
pub fn check_balanced<T: Ord>(root: &Node<T>) -> bool {
  check_balanced_helper(root).is_some()
}

// This function takes in a node and returns its height if balanced, or None if unbalanced
fn check_balanced_helper<T: Ord>(root: &Node<T>) -> Option<usize> {
  match (&root.left_child, &root.right_child) {
    // The root has both children
    (Some(l_child), Some(r_child)) => {
      // Take the children's height, if either is unbalanced, propagate the message
      let l_height = check_balanced_helper(&l_child)?;
      let r_height = check_balanced_helper(&r_child)?;

      // Both children are balanced
      if l_height <= r_height + 1 && r_height <= l_height + 1 {
        // Children are of similar height
        Some(cmp::max(l_height, r_height) + 1)
      } else {
        // One child is much taller than the other
        None
      }
    }

    // The root has exactly one child
    (None, Some(child)) | (Some(child), None) => {
      let height = check_balanced_helper(&child)?;

      if height <= 1 {
        // The balanced child is short enough
        Some(height + 1)
      } else {
        // The balanced child is too tall
        None
      }
    }

    // The root has no children -> is balanced
    (None, None) => Some(1),
  }
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
    assert!(check_balanced(&root));
    root.add_node(lll_ggrandchild);
    assert!(!check_balanced(&root));
    root.add_node(lr_grandchild);

    assert!(!check_balanced(&root));
  }
}
