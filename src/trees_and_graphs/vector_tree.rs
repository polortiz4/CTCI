type NodeId = usize;

#[derive(Clone)]
pub struct Node<T> {
    pub data: T,

    children: Vec<NodeId>,
    tree_ptr: *mut Tree<T>,
    idx: NodeId,
}
impl<T: std::cmp::PartialEq> Node<T> {
    pub fn get_children(&self) -> Vec<&Node<T>> {
        self.children
            .iter()
            .map(|idx| unsafe {
                (*self.tree_ptr)
                    .get(*idx)
                    .expect("Children is not in the Tree")
            })
            .collect()
    }

    pub fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    pub fn get_children_mut(&self) -> Vec<&mut Node<T>> {
        self.children
            .iter()
            .map(|idx| unsafe {
                (*self.tree_ptr)
                    .get_mut(*idx)
                    .expect("Children is not in the Tree")
            })
            .collect()
    }

    pub fn add_new_child(&mut self, child_data: T) {
        let new_node = Node {
            data: child_data,
            children: Vec::new(),
            tree_ptr: self.tree_ptr,
            idx: unsafe { (*self.tree_ptr).next_idx() },
        };
        self.add_child(&new_node);
        unsafe {
            (*self.tree_ptr).push_node(new_node);
            // Warning! This Node might have gotten moved here, so no fields are valid here,
        }
    }
    fn add_child(&mut self, child: &Node<T>) {
        self.children.push(child.idx);
    }
}

pub struct Tree<T> {
    nodes: Vec<Node<T>>,
}

pub struct NotEmptyError;
impl<T: std::cmp::PartialEq> Tree<T> {
    pub fn new() -> Self {
        Tree { nodes: Vec::new() }
    }

    pub fn set_root(&mut self, data: T) -> Result<(), NotEmptyError> {
        match self.get(0) {
            Some(_) => Err(NotEmptyError),
            None => {
                let new_node = Node {
                    data,
                    children: Vec::new(),
                    tree_ptr: self,
                    idx: 0,
                };
                self.push_node(new_node);
                Ok(())
            }
        }
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
    pub fn root_mut(&mut self) -> Option<&mut Node<T>> {
        self.nodes.get_mut(0)
    }
    pub fn find(&self, data: T) -> Option<&Node<T>> {
        let curr = self.nodes.get(0)?;
        self.find_helper(data, curr)
    }
    fn find_helper<'a>(&self, data: T, curr: &'a Node<T>) -> Option<&'a Node<T>> {
        if curr.data == data {
            return Some(curr);
        } else {
            for child in curr.get_children() {
                return self.find_helper(data, child);
            }
            return None;
        }
    }
}
impl<T: Copy + std::cmp::PartialEq> Tree<T> {
    pub fn all_paths(&self) -> Vec<Vec<T>> {
        let mut tree_ans = Vec::new();
        let node = self.root();
        if let Some(root) = node {
            self.all_paths_helper(root, &mut tree_ans, &mut Vec::new());
        }
        tree_ans
    }
    fn all_paths_helper(&self, node: &Node<T>, ans: &mut Vec<Vec<T>>, path: &Vec<T>) {
        let mut path_copy = path.to_vec();
        path_copy.push(node.data);
        if node.is_leaf() {
            ans.push(path_copy);
        } else {
            for child in node.get_children() {
                self.all_paths_helper(child, ans, &path_copy);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tree_test() {
        let mut tree = Tree::<i32>::new();
        let _res = tree.set_root(0);
        tree.root_mut().unwrap().add_new_child(-1);
        tree.root_mut().unwrap().add_new_child(3);

        tree.root_mut()
            .unwrap()
            .get_children_mut()
            .get_mut(1)
            .unwrap()
            .add_new_child(1);
        tree.root_mut()
            .unwrap()
            .get_children_mut()
            .get_mut(1)
            .unwrap()
            .add_new_child(4);
        tree.root_mut()
            .unwrap()
            .get_children_mut()
            .get_mut(1)
            .unwrap()
            .add_new_child(5);

        let r_children: Vec<i32> = tree
            .root_mut()
            .unwrap()
            .get_children()
            .iter()
            .map(|x| x.data)
            .collect();
        assert_eq!(vec![-1, 3], r_children);

        let t_children: Vec<i32> = tree
            .root_mut()
            .unwrap()
            .get_children_mut()
            .get_mut(1)
            .unwrap()
            .get_children()
            .iter()
            .map(|x| x.data)
            .collect();
        assert_eq!(vec![1, 4, 5], t_children);

        //       0
        //     /   \
        //   -1     3
        //        / | \
        //       1  5  4
        let node_data: Vec<i32> = tree.nodes.iter().map(|x| x.data).collect();
        assert_eq!(vec![0, -1, 3, 1, 4, 5], node_data);
    }

    #[test]
    fn test_vectorization() {
        let mut tree = Tree::<i32>::new();
        let _res = tree.set_root(0);
        tree.root_mut().unwrap().add_new_child(-1);
        tree.root_mut().unwrap().add_new_child(3);

        let ans = tree.all_paths();
        let expected = vec![vec![0, -1], vec![0, 3]];
        assert_eq!(expected, ans);
    }
}
