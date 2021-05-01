// Solutions to Exercise 4.8

use crate::trees_and_graphs::vector_bst::Node;


/// Given two nodes in a tree, find their first common ancestor assuming the have a pointer to their parent node
/// 
/// # Example:
/// 
/// ```
/// use ctci::trees_and_graphs::vector_bst::DoubleLinkedBST;
/// use ctci::trees_and_graphs::first_common_ancestor::first_common_ancestor;
/// 
/// let mut tree = DoubleLinkedBST::<i32>::new();
/// tree.add_new_node(5);
/// tree.add_new_node(6);
/// tree.add_new_node(3);
/// 
/// let root = tree.root().unwrap();
/// let l_child = root.get_left_child().unwrap();
/// let r_child = root.get_right_child().unwrap();
/// 
/// assert_eq!(first_common_ancestor(&l_child, &r_child).unwrap() as *const _, root as *const _ );
/// ```
pub fn first_common_ancestor<'a, T>(
    node1: &'a Node<T>,
    node2: &'a Node<T>,
) -> Option<&'a Node<T>> {
    let depth1 = node_depth(&node1);
    let depth2 = node_depth(&node2);
    let delta = abs_diff(depth1, depth2);
    let larger_node;
    let mut shorter_node;

    if depth1 > depth2 {
        larger_node = node1;
        shorter_node = node2;
    } else {
        shorter_node = node1;
        larger_node = node2;
    }

    let mut chopped = chop(&larger_node, delta);

    while chopped as *const _ != shorter_node as *const _ {
        match (chopped.get_mut_parent(), shorter_node.get_mut_parent()) {
            (Some(c_parent), Some(s_parent)) => {
                chopped = &*c_parent;
                shorter_node = &*s_parent;
            }
            _ => return None,
        }
    }
    Some(chopped)
}
fn chop<'a, T>(node: &'a Node<T>, delta: usize) -> &'a Node<T> {
    let mut result: &'a Node<T> = node;
    for _ in 0..delta {
        result = &*result.get_mut_parent().expect(&format!(
            "can't chop off tree, its height is less than {}",
            delta
        ));
    }
    result
}
fn abs_diff(a: usize, b: usize) -> usize {
    (a as isize - b as isize).unsigned_abs()
}
fn node_depth<T>(node: &Node<T>) -> usize {
    let mut depth = 1;
    let mut curr = node;
    while let Some(parent_ptr) = curr.get_mut_parent() {
        depth += 1;
        curr = &*parent_ptr;
    }
    depth
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::trees_and_graphs::vector_bst::DoubleLinkedBST;

    #[test]
    fn main_test() {
        let mut tree = DoubleLinkedBST::<i32>::new();
        tree.add_new_node(5);
        tree.add_new_node(6);
        tree.add_new_node(3);
        tree.add_new_node(-1);
        tree.add_new_node(4);

        let root = tree.root().unwrap();
        let l_child = root.get_left_child().unwrap();
        let lr_child = l_child.get_right_child().unwrap();
        let ll_child = l_child.get_left_child().unwrap();

        assert_eq!(first_common_ancestor(&ll_child, &lr_child).unwrap() as *const _, l_child as *const _);
    }
}
