// Solution to Exercise 4.10
use crate::trees_and_graphs::vector_bst::DoubleLinkedBST;
use crate::trees_and_graphs::vector_bst::Node;


/// Given two trees, check if one is a subset of the other
/// 
/// Examples:
/// 
/// ```
/// use ctci::trees_and_graphs::vector_bst::DoubleLinkedBST;
/// use ctci::trees_and_graphs::check_subtree::check_subtree;
/// 
/// let mut big_tree = DoubleLinkedBST::<i32>::new();
/// big_tree.add_new_node(0);
/// big_tree.add_new_node(1);
/// 
/// let mut small_tree = DoubleLinkedBST::<i32>::new();
/// small_tree.add_new_node(1);
/// 
/// assert!(check_subtree(&big_tree, &small_tree));
/// ```
pub fn check_subtree<T: Ord>(t1: &DoubleLinkedBST<T>, t2: &DoubleLinkedBST<T>) -> bool {
    let t1_root = t1.root();
    let t2_root = t2.root();
    check_subtree_helper(t1_root, t2_root)
}
fn check_subtree_helper<T: Ord>(t1: Option<&Node<T>>, t2: Option<&Node<T>>) -> bool {
    if check_node(t1, t2) {
        true
    } else {
        match (t1, t2) {
            (Some(t1r), Some(_)) => {
                check_subtree_helper(t1r.get_right_child(), t2)
                    || check_subtree_helper(t1r.get_left_child(), t2)
            }
            (None, None) => true,
            _ => false,
        }
    }
}
fn check_node<T: Ord>(t1: Option<&Node<T>>, t2: Option<&Node<T>>) -> bool {
    match (t1, t2) {
        (Some(t1r), Some(t2r)) => {
            if t1r.data != t2r.data {
                return false;
            }
            check_node(t1r.get_right_child(), t2r.get_right_child())
                && check_node(t1r.get_left_child(), t2r.get_left_child())
        }
        (None, None) => true,
        _ => false,
    }
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn valid_subtree_test(){
        let mut big_tree = DoubleLinkedBST::<i32>::new();
        big_tree.add_new_node(0);
        big_tree.add_new_node(1);
        big_tree.add_new_node(-2);
        big_tree.add_new_node(2);
        big_tree.add_new_node(-1);
        
        let mut small_tree = DoubleLinkedBST::<i32>::new();
        small_tree.add_new_node(1);
        small_tree.add_new_node(2);
        
        assert!(check_subtree(&big_tree, &small_tree));
    }
    
    #[test]
    fn invalid_subtree_test(){
        let mut big_tree = DoubleLinkedBST::<i32>::new();
        big_tree.add_new_node(0);
        big_tree.add_new_node(1);
        big_tree.add_new_node(-2);
        big_tree.add_new_node(2);
        big_tree.add_new_node(-1);
        
        let mut small_tree = DoubleLinkedBST::<i32>::new();
        small_tree.add_new_node(0);
        small_tree.add_new_node(1);
        small_tree.add_new_node(2);
        assert!(!check_subtree(&big_tree, &small_tree));
    }
}