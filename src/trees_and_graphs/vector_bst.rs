type NodeId = usize;

#[derive(Clone, Copy)]
pub struct Node<T> {
    parent_idx: Option<NodeId>,
    right_idx: Option<NodeId>,
    left_idx: Option<NodeId>,

    pub data: T,
    tree_ptr: *mut DoubleLinkedBST<T>,
    idx: NodeId,
}

impl<T> Node<T> {
    pub fn new(data: T, idx: NodeId, tree: *mut DoubleLinkedBST<T>) -> Self {
        Node {
            parent_idx: None,
            right_idx: None,
            left_idx: None,
            tree_ptr: tree,
            idx,
            data,
        }
    }

    pub fn new_child(&self, data: T, idx: NodeId, parent_idx: NodeId) -> Self {
        let mut node = Node::new(data, idx, self.tree_ptr);
        node.parent_idx = Some(parent_idx);
        node
    }

    pub fn get_right_child<'a>(&self) -> Option<&'a Node<T>> {
        unsafe {
            match self.right_idx {
                Some(idx) => Some(
                    (*self.tree_ptr)
                        .get(idx)
                        .expect("Right child not held by vec"),
                ),
                None => None,
            }
        }
    }
    pub fn get_left_child<'a>(&self) -> Option<&'a Node<T>> {
        unsafe {
            match self.left_idx {
                Some(idx) => Some(
                    (*self.tree_ptr)
                        .get(idx)
                        .expect("Left child not held by vec"),
                ),
                None => None,
            }
        }
    }
    pub(crate) fn get_mut_parent<'a>(&self) -> Option<&'a mut Node<T>> {
        unsafe {
            match self.parent_idx {
                Some(idx) => Some(
                    (*self.tree_ptr)
                        .get_mut(idx)
                        .expect("Parent not held by vec"),
                ),
                None => None,
            }
        }
    }
    pub fn get_parent<'a>(&self) -> Option<&'a Node<T>> {
        unsafe {
            match self.parent_idx {
                Some(idx) => Some((*self.tree_ptr).get(idx).expect("Parent not held by vec")),
                None => None,
            }
        }
    }
    pub(crate) fn get_mut_right_child<'a>(&self) -> Option<&'a mut Node<T>> {
        unsafe {
            match self.right_idx {
                Some(idx) => Some(
                    (*self.tree_ptr)
                        .get_mut(idx)
                        .expect("Right child not held by vec"),
                ),
                None => None,
            }
        }
    }
    pub(crate) fn get_mut_left_child<'a>(&self) -> Option<&'a mut Node<T>> {
        unsafe {
            match self.left_idx {
                Some(idx) => Some(
                    (*self.tree_ptr)
                        .get_mut(idx)
                        .expect("Left child not held by vec"),
                ),
                None => None,
            }
        }
    }
}

impl<T: Ord> Node<T> {
    pub fn add_new_node(&mut self, data: T) -> NodeId {
        if self.data < data {
            match self.get_mut_right_child() {
                Some(r_child) => r_child.add_new_node(data),
                None => {
                    unsafe {
                        let new_node = self.new_child(data, (*self.tree_ptr).next_idx(), self.idx);
                        self.right_idx = Some((*self.tree_ptr).next_idx());
                        let new_idx = (*self.tree_ptr).push_node(new_node);
                        // Warning! This Node might have gotten moved here, so no fields are valid here,
                        new_idx
                    }
                }
            }
        } else {
            match self.get_mut_left_child() {
                Some(l_child) => l_child.add_new_node(data),
                None => {
                    unsafe {
                        let new_node = self.new_child(data, (*self.tree_ptr).next_idx(), self.idx);
                        self.left_idx = Some((*self.tree_ptr).next_idx());
                        let new_idx = (*self.tree_ptr).push_node(new_node);
                        // Warning! This Node might have gotten moved here, so no fields are valid here,
                        new_idx
                    }
                }
            }
        }
    }
}

pub struct DoubleLinkedBST<T> {
    nodes: Vec<Node<T>>,
}

impl<T: PartialEq + Copy> DoubleLinkedBST<T>{
    pub fn find_idx(&self, data: T) -> Option<NodeId> {
        self.nodes.iter().position(|&r| r.data == data)
    }
}
impl<T: Ord> DoubleLinkedBST<T> {
    pub fn add_new_node(&mut self, data: T) -> NodeId {
        match self.nodes.get_mut(0) {
            Some(root) => root.add_new_node(data),
            None => {
                let new_node = Node::new(data, 0, self);
                self.push_node(new_node)
            }
        }
    }

    pub fn find(&self, data: T) -> Option<&Node<T>> {
        self.find_helper(data, self.nodes.get(0)?)
    }
    fn find_helper<'a>(&self, data: T, curr: &'a Node<T>) -> Option<&'a Node<T>> {
        if curr.data == data {
            return Some(curr);
        } else if curr.data < data {
            self.find_helper(data, curr.get_left_child()?)
        } else {
            self.find_helper(data, curr.get_right_child()?)
        }
    }
}
impl<T> DoubleLinkedBST<T> {
    pub fn new() -> Self {
        DoubleLinkedBST { nodes: Vec::new() }
    }

    pub(super) fn get(&self, id: NodeId) -> Option<&Node<T>> {
        self.nodes.get(id)
    }
    pub(super) fn get_mut(&mut self, id: NodeId) -> Option<&mut Node<T>> {
        self.nodes.get_mut(id)
    }
    pub(super) fn next_idx(&self) -> NodeId {
        self.nodes.len()
    }
    pub(super) fn push_node(&mut self, node: Node<T>) -> NodeId {
        self.nodes.push(node);
        self.next_idx() - 1
    }
    pub fn root(&self) -> Option<&Node<T>> {
        self.nodes.get(0)
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

        let root = tree.root().unwrap();
        let rchild = root.get_right_child().unwrap();
        let lchild = root.get_left_child().unwrap();
        let lrchild = lchild.get_right_child().unwrap();
        let rrchild = rchild.get_right_child().unwrap();

        assert_eq!(root.data, 0);
        assert_eq!(rrchild.data, 2);
        assert_eq!(lrchild.data, -1);
    }
}
