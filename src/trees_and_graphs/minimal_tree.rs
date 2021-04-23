use crate::trees_and_graphs::binary_search_tree::Node;
// Solution to Excercice 4.2

/// Takes a sorted (increasing order) array with unique integer elements and creates a binary tree with minimanl height from it
///
/// # Examples:
///
/// ```
/// use ctci::trees_and_graphs::minimal_tree::minimal_tree;
/// let my_vec = vec![-1, 0, 1];
/// let root = minimal_tree(&my_vec);
///
/// assert_eq!(root.data, 0);
/// assert_eq!(root.right_child.as_ref().unwrap().data, 1);
/// assert_eq!(root.left_child.as_ref().unwrap().data, -1);
///
/// ```
pub fn minimal_tree(array: &Vec<i32>) -> Node<i32> {
  let size = array.len();
  let center_idx = size / 2;
  let mut root = Node::new(array[center_idx]);
  minimal_tree_helper(&mut root, &array[0..center_idx]);
  minimal_tree_helper(&mut root, &array[center_idx..]);
  root
}

fn minimal_tree_helper(root: &mut Node<i32>, array: &[i32]) {
  let size = array.len();
  let center_idx = size / 2;
  let new_node = Node::new(array[center_idx]);
  root.add_node(new_node);
  if size == 2 {
    let new_leaf = Node::new(array[0]);
    root.add_node(new_leaf);
    return;
  } else if size == 1 {
    assert_eq!(1, size);
    return;
  }
  minimal_tree_helper(root, &array[0..center_idx]);
  minimal_tree_helper(root, &array[center_idx..]);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn main_test() {
    let my_vec = vec![-1, 0, 1, 3, 4, 6];
    let root = minimal_tree(&my_vec);

    assert_eq!(root.data, 3);
    assert_eq!(root.left_child.as_ref().unwrap().data, 0);
    assert_eq!(root.right_child.as_ref().unwrap().data, 4);
    assert_eq!(
      root
        .right_child
        .as_ref()
        .unwrap()
        .right_child
        .as_ref()
        .unwrap()
        .data,
      6
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
      1
    );
    assert_eq!(
      root
        .left_child
        .as_ref()
        .unwrap()
        .left_child
        .as_ref()
        .unwrap()
        .data,
      -1
    );
  }
}
