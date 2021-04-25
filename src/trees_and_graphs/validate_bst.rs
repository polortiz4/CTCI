// Solution to Exercise 4.5
use crate::trees_and_graphs::binary_search_tree::Tree;


/// Checks if a tree is a binary tree
/// 
/// # Examples
/// 
/// ```
/// use ctci::trees_and_graphs::validate_bst::validate_bst;
/// use ctci::trees_and_graphs::binary_search_tree::Node;
/// 
/// //non-BST
/// let mut root = Node::new(4);
/// root.left_child = Some(Box::new(Node::new(2)));
/// root.right_child = Some(Box::new(Node::new(2)));
/// 
/// assert!(!validate_bst(&Some(Box::new(root.clone()))));
/// 
/// //BST
/// let mut root = Node::new(4);
/// root.left_child = Some(Box::new(Node::new(2)));
/// root.right_child = Some(Box::new(Node::new(5)));
/// 
/// assert!(validate_bst(&Some(Box::new(root.clone()))));
/// ```

pub fn validate_bst<T: Ord>(tree: &Tree<T>) -> bool {
  validate_bst_helper(tree).is_some()
}
fn validate_bst_helper<T: Ord>(tree: &Tree<T>) -> Option<Option<&T>> {
  match tree {
    None => Some(None),
    Some(node) => {
      let l_option = validate_bst_helper(&node.left_child)?;
      let r_option = validate_bst_helper(&node.right_child)?;

      // Both children are BST
      match (l_option, r_option) {
        // Children are not empty
        (Some(l_value), Some(r_value)) => {
          if l_value >= &node.data || &node.data >= r_value {
            None
          } else {
            Some(Some(&node.data))
          }
        }
        // At least one of the children is empty
        _ => Some(Some(&node.data)),
      }
    }
  }
}

#[cfg(test)]
mod tests{
  use crate::trees_and_graphs::binary_search_tree::Node;
  use super::*;
  
  #[test]
  fn main_valid_bst(){
    let mut root = Node::new(4);
    let l_child = Node::new(2);
    let r_child = Node::new(5);
    let ll_grandchild = Node::new(1);
    let lll_ggrandchild = Node::new(0);
    let lr_grandchild = Node::new(3);
    root.add_node(l_child);
    root.add_node(r_child);
    root.add_node(ll_grandchild);
    root.add_node(lll_ggrandchild);
    root.add_node(lr_grandchild);

    assert!(validate_bst(&Some(Box::new(root.clone()))));

    let mut root = Node::new(4);
    root.left_child = Some(Box::new(Node::new(2)));
    assert!(validate_bst(&Some(Box::new(root.clone()))));
    
    let mut n_node = Node::new(3);
    n_node.right_child = Some(Box::new(Node::new(5)));
    root.right_child = Some(Box::new(n_node));
    assert!(!validate_bst(&Some(Box::new(root.clone()))));
  }
  #[test]
  fn main_invalid_bst(){
    let mut root = Node::new(4);
    root.left_child = Some(Box::new(Node::new(2)));
    root.right_child = Some(Box::new(Node::new(2)));

    assert!(!validate_bst(&Some(Box::new(root.clone()))));
  }
}