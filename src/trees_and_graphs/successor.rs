// Solutions to Exercise 4.6

use crate::trees_and_graphs::double_linked_bst::DoubleLinkedNode;

/// For a given node in a bst, find the immediate successor
///
/// # Examples:
///
/// ```
/// use ctci::trees_and_graphs::successor::successor;
/// use ctci::trees_and_graphs::double_linked_bst::DoubleLinkedNode;
/// 
/// let mut root = DoubleLinkedNode::new(0);
/// root.add_new_node(6);
/// root.add_new_node(4);
/// assert_eq!(4, successor(&root));
/// ```
pub fn successor<'a, T: Copy + Ord>(node: &'a DoubleLinkedNode<'a, T>) -> T {
    // check if we have right child
    if let Some(r_tree) = &node.right_child {
        return find_leftmost_descendant(&r_tree.borrow());
    }

    let ascendant;
    unsafe {
        ascendant = youngest_greater_parent(node, node.data);
    }
    match ascendant {
        Some(parent) => parent.data,
        None => node.data,
    }
}
unsafe fn youngest_greater_parent<'a, T: Copy + Ord>(
    node: &'a DoubleLinkedNode<'a, T>,
    ref_value: T,
) -> Option<&'a DoubleLinkedNode<'a, T>> {
    if node.data > ref_value {
        Some(node)
    } else {
        let parent = node.parent?;
        youngest_greater_parent(&*parent, ref_value)
    }
}
fn find_leftmost_descendant<T: Copy + Ord>(node: &DoubleLinkedNode<T>) -> T {
    match &node.left_child {
        Some(l_child) => find_leftmost_descendant(&l_child.borrow()),
        None => node.data,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn main_test() {
        let mut root = DoubleLinkedNode::new(0);
        root.add_new_node(6);
        root.add_new_node(4);
        root.add_new_node(3);
        root.add_new_node(-2);
        assert_eq!(3, successor(&root));

        let mut leaf = DoubleLinkedNode::new(-1);
        leaf.parent = Some(&*root.left_child.as_ref().unwrap().borrow());
        assert_eq!(0, successor(&leaf));

        let mut leaf = DoubleLinkedNode::new(-3);
        leaf.parent = Some(&*root.left_child.as_ref().unwrap().borrow());
        assert_eq!(-2, successor(&leaf));

        let mut leaf = DoubleLinkedNode::new(7);
        leaf.parent = Some(&*root.right_child.as_ref().unwrap().borrow());
        assert_eq!(7, successor(&leaf));

        let mut leaf = DoubleLinkedNode::new(5);
        leaf.parent = Some(&*root.right_child.as_ref().unwrap().borrow().left_child.as_ref().unwrap().borrow());
        assert_eq!(6, successor(&leaf));
    }
}
