// Solution to Exercise 4.12
use crate::trees_and_graphs::vector_bst::DoubleLinkedBST;
use crate::trees_and_graphs::vector_bst::Node;


/// Given a BST and an integer value, determine the number of paths it takes to get there.
/// 
/// # Examples:
/// ```
/// use ctci::trees_and_graphs::paths_with_sum::paths_with_sum;
/// use ctci::trees_and_graphs::vector_bst::DoubleLinkedBST;
/// 
/// let mut tree = DoubleLinkedBST::<i32>::new();
/// tree.add_new_node(1);
/// tree.add_new_node(2);
/// 
/// assert_eq!(0, paths_with_sum(&tree, 0));
/// assert_eq!(1, paths_with_sum(&tree, 1));
/// assert_eq!(1, paths_with_sum(&tree, 2));
/// assert_eq!(1, paths_with_sum(&tree, 3));
/// assert_eq!(0, paths_with_sum(&tree, 4));
/// ```
pub fn paths_with_sum(tree: &DoubleLinkedBST<i32>, sum_total: i32) -> usize {
    let root = tree.root();
    let mut res = 0;
    node_recurse(root, sum_total, &mut res)
}
fn node_recurse(node: Option<&Node<i32>>, sum_total: i32, res: &mut usize) -> usize {
    if path_helper(node, sum_total) {
        *res += 1;
    } else {
        match node {
            Some(node) => {
                node_recurse(node.get_left_child(), sum_total, res);
                node_recurse(node.get_right_child(), sum_total, res);
            }
            None => {},
        }
    }
    *res
}
fn path_helper(node: Option<&Node<i32>>, remainder: i32) -> bool {
    match node {
        Some(node) => {
            if node.data == remainder {
                true
            } else {
                path_helper(node.get_left_child(), remainder - node.data)
                    || path_helper(node.get_right_child(), remainder - node.data)
            }
        }
        None => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn path_sum() {
        let mut tree = DoubleLinkedBST::<i32>::new();
        tree.add_new_node(0);
        tree.add_new_node(1);
        tree.add_new_node(-2);
        tree.add_new_node(2);
        tree.add_new_node(-1);

        assert_eq!(1, paths_with_sum(&tree, 0));
        assert_eq!(1, paths_with_sum(&tree, 1));
        assert_eq!(1, paths_with_sum(&tree, -1));
        assert_eq!(1, paths_with_sum(&tree, -2));
        assert_eq!(1, paths_with_sum(&tree, 2));
        assert_eq!(1, paths_with_sum(&tree, 3));
        assert_eq!(0, paths_with_sum(&tree, 4));
    }
}
