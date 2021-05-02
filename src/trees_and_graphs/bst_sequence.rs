// Solution to Exercise 4.9
use super::vector_bst::DoubleLinkedBST;
use super::vector_tree::Tree;

type NodeId = usize;
use std::cmp::PartialEq;

/// Given a Binary search tree, return all possible insertion orders that could lead to it
/// 
/// # Examples:
/// ```
/// use ctci::trees_and_graphs::vector_bst::DoubleLinkedBST;
/// use ctci::trees_and_graphs::vector_tree::Tree;
/// use ctci::trees_and_graphs::bst_sequence::bst_sequence;
/// 
/// let mut tree = DoubleLinkedBST::<i32>::new();
/// 
/// tree.add_new_node(0);
/// tree.add_new_node(1);
/// tree.add_new_node(-2);
/// 
/// let ans = bst_sequence(&tree);
/// let expected = vec![
///     vec![0, -2, 1],
///     vec![0, 1, -2],
/// ];
/// assert_eq!(expected, ans);
/// ```
pub fn bst_sequence<T: PartialEq + Copy + Clone>(tree: &DoubleLinkedBST<T>) -> Vec<Vec<T>> {
    let mut tree_ans = Tree::<T>::new();
    let _res = tree_ans.set_root(tree.get(0).expect("Invalid id for bst").data);
    let valid_next = Vec::new();
    bst_sequence_helper(tree, 0, valid_next, &mut tree_ans, 0);
    tree_ans.all_paths()
}
fn bst_sequence_helper<T: PartialEq + Copy + Clone>(
    tree: &DoubleLinkedBST<T>,
    node_id: NodeId,
    valid_next: Vec<T>,
    tree_ans: &mut Tree<T>,
    last_node_idx: NodeId,
) {
    let node = tree.get(node_id).expect("Invalid id for bst");
    let left_child = node.get_left_child();
    let right_child = node.get_right_child();
    let mut valid_next = valid_next.to_vec();

    match (left_child, right_child) {
        (Some(l_child), Some(r_child)) => {
            valid_next.push(l_child.data);
            valid_next.push(r_child.data);
        }
        (Some(child), None) | (None, Some(child)) => valid_next.push(child.data),
        _ => {}
    }

    let valid_next_copy = valid_next.to_vec();
    for next in valid_next {
        let index = valid_next_copy
            .iter()
            .position(|x| *x == next)
            .expect("Error here somehow");
        let mut valid_minus_curr = valid_next_copy.to_vec();
        valid_minus_curr.remove(index);
        let next_idx = tree.find_idx(next).expect("bst doesn't have the next node");
        tree_ans
            .get_mut(last_node_idx)
            .expect("Tree doesn't have the expected node")
            .add_new_child(next);
        bst_sequence_helper(
            tree,
            next_idx,
            valid_minus_curr,
            tree_ans,
            tree_ans.next_idx() - 1,
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bst_test() {
        let mut tree = DoubleLinkedBST::<i32>::new();
        tree.add_new_node(0);
        tree.add_new_node(1);
        tree.add_new_node(-2);
        tree.add_new_node(2);
        tree.add_new_node(-1);

        let ans = bst_sequence(&tree);
        let expected = vec![
            vec![0, -2, 1, -1, 2],
            vec![0, -2, 1, 2, -1],
            vec![0, -2, -1, 1, 2],
            vec![0, 1, -2, 2, -1],
            vec![0, 1, -2, -1, 2],
            vec![0, 1, 2, -2, -1],
        ];
        assert_eq!(expected, ans);
    }
}
