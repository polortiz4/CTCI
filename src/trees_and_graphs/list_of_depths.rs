use crate::trees_and_graphs::binary_search_tree::Node;
use std::collections::LinkedList;

// Solution to Excercise 4.3


/// Given a binary tree, build a vector of linked lists where each list corresponds to the level
/// 
/// # Example
/// ```
/// use std::collections::LinkedList;
/// use ctci::trees_and_graphs::list_of_depths::list_of_depths;
/// use ctci::trees_and_graphs::binary_search_tree::Node;
/// 
/// let mut root = Node::new(4);
/// let l_child = Node::new(2);
/// 
/// root.add_node(l_child);
/// 
/// let answer = list_of_depths(&root);
/// 
/// let mut level_list = LinkedList::new();
/// level_list.push_back(4);
/// assert_eq!(*answer.get(0).unwrap(), level_list);
/// 
/// let mut level_list = LinkedList::new();
/// level_list.push_back(2);
/// assert_eq!(*answer.get(1).unwrap(), level_list);
/// ```
pub fn list_of_depths<T: Ord + Copy>(tree: &Node<T>) -> Vec<LinkedList<T>> {
  let mut answer: Vec<LinkedList<T>> = Vec::new();
  list_of_depths_helper(tree, &mut answer, 0);
  answer
}

fn list_of_depths_helper<T>(tree: &Node<T>, vec_list: &mut Vec<LinkedList<T>>, level: usize)
where
  T: Ord + Copy,
{
  if vec_list.len() == level {
    vec_list.push(LinkedList::new());
  }
  if let Some(list) = vec_list.get_mut(level) {
      list.push_back(tree.data);
  }
  if let Some(node) = &tree.left_child {
    list_of_depths_helper(&node, vec_list, level + 1);
  }
  if let Some(node) = &tree.right_child {
    list_of_depths_helper(&node, vec_list, level + 1);
  }
}

#[cfg(test)]
mod tests{
  use super::*;

  #[test]
  fn main_test(){
    let mut root = Node::new(4);
    let l_child = Node::new(2);
    let r_child = Node::new(5);
    let ll_grandchild = Node::new(1);
    let lr_grandchild = Node::new(3);

    root.add_node(l_child);
    root.add_node(r_child);
    root.add_node(lr_grandchild);
    root.add_node(ll_grandchild);

    let answer = list_of_depths(&root);

    let mut level_list = LinkedList::new();
    level_list.push_back(4);
    assert_eq!(*answer.get(0).unwrap(), level_list);
    
    let mut level_list = LinkedList::new();
    level_list.push_back(2);
    level_list.push_back(5);
    assert_eq!(*answer.get(1).unwrap(), level_list);
    
    let mut level_list = LinkedList::new();
    level_list.push_back(1);
    level_list.push_back(3);
    assert_eq!(*answer.get(2).unwrap(), level_list);
  }
}