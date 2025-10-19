use std::cmp::{max, min};

pub struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    pub fn add_left_child(&mut self, parent_id: usize, key: u32) -> usize {
        self.add_node(parent_id, key, true)
    }

    pub fn add_right_child(&mut self, parent_id: usize, key: u32) -> usize {
        self.add_node(parent_id, key, false)
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left  child of the  
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics    ///  if the `parent_id` does not exist, or if the node `parent_id ` has
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert_eq!(
                self.nodes[parent_id].id_left, None,
                "Parent node has the left child already set"
            );
        } else {
            assert_eq!(
                self.nodes[parent_id].id_right, None,
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }
}

fn is_bst(tree: &Tree) -> bool {
    let (is_bst, _, _) = rec_is_bst(tree, Some(0));
    is_bst
}

fn rec_is_bst(tree: &Tree, id: Option<usize>) -> (bool, u32, u32) {
    match id {
        None => (true, u32::MAX, u32::MIN),
        Some(id) => {
            assert!(id < tree.nodes.len(), "Node id is out of range");
            let c_node = &tree.nodes[id];
            let (l_is_bst, l_min_val, l_max_val) = rec_is_bst(tree, c_node.id_left);
            let (r_is_bst, r_min_val, r_max_val) = rec_is_bst(tree, c_node.id_right);
            let is_bst = l_is_bst && r_is_bst && c_node.key >= l_max_val && c_node.key < r_min_val;
            (
                is_bst,
                min(c_node.key, min(l_min_val, r_min_val)),
                max(c_node.key, max(r_max_val, l_max_val)),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_one_tree() {
        let mut tree = Tree::with_root(10);
        tree.add_left_child(0, 5);
        tree.add_right_child(0, 22);
        assert!(is_bst(&tree));
    }

    #[test]
    fn test_depth_two_tree() {
        let mut tree = Tree::with_root(10);
        let id_left = tree.add_left_child(0, 5);
        tree.add_left_child(id_left, 4);
        tree.add_right_child(id_left, 8);
        let id_right = tree.add_right_child(0, 22);
        tree.add_left_child(id_right, 12);
        tree.add_right_child(id_right, 30);
        assert!(is_bst(&tree));
    }

    #[test]
    fn test_typical_counter_example() {
        let mut tree = Tree::with_root(10);
        let id_left = tree.add_left_child(0, 5);
        tree.add_right_child(id_left, 12);
        assert!(!is_bst(&tree));
    }

    #[test]
    fn test_complex_true_bst_not_full() {
        let mut tree = Tree::with_root(15);

        // Left subtree
        let id_l = tree.add_left_child(0, 7);
        let id_l_l = tree.add_left_child(id_l, 3);
        let id_l_r = tree.add_right_child(id_l, 9);

        tree.add_left_child(id_l_l, 2);
        tree.add_right_child(id_l_l, 5);

        tree.add_left_child(id_l_r, 8);
        tree.add_right_child(id_l_r, 10);

        // Right subtree
        let id_r = tree.add_right_child(0, 23);
        let id_r_l = tree.add_left_child(id_r, 19);
        tree.add_right_child(id_r, 31);

        tree.add_left_child(id_r_l, 17);
        tree.add_right_child(id_r_l, 21);

        assert!(is_bst(&tree));
    }

    #[test]
    fn test_complex_true_bst() {
        let mut tree = Tree::with_root(15);

        // Left subtree
        let id_l = tree.add_left_child(0, 7);
        let id_l_l = tree.add_left_child(id_l, 3);
        let id_l_r = tree.add_right_child(id_l, 9);

        tree.add_left_child(id_l_l, 2);
        tree.add_right_child(id_l_l, 5);

        tree.add_left_child(id_l_r, 8);
        tree.add_right_child(id_l_r, 10);

        // Right subtree
        let id_r = tree.add_right_child(0, 23);
        let id_r_l = tree.add_left_child(id_r, 19);
        let id_r_r = tree.add_right_child(id_r, 31);

        tree.add_left_child(id_r_l, 17);
        tree.add_right_child(id_r_l, 21);

        tree.add_left_child(id_r_r, 29);
        tree.add_right_child(id_r_r, 41);

        assert!(is_bst(&tree));
    }

    #[test]
    fn test_complex_false_bst_ancestor_violation() {
        let mut tree = Tree::with_root(15);

        // Left subtree
        let id_l = tree.add_left_child(0, 7);
        let id_l_l = tree.add_left_child(id_l, 3);
        let id_l_r = tree.add_right_child(id_l, 9);

        tree.add_left_child(id_l_l, 2);
        tree.add_right_child(id_l_l, 5);

        tree.add_left_child(id_l_r, 8);
        tree.add_right_child(id_l_r, 10);

        // Right subtree with the violation
        let id_r = tree.add_right_child(0, 23);
        let id_r_l = tree.add_left_child(id_r, 19);
        let id_r_r = tree.add_right_child(id_r, 31);

        // Here's the subtle violation (should be > 15 because it's in root's right subtree)
        tree.add_left_child(id_r_l, 14);
        tree.add_right_child(id_r_l, 21);

        tree.add_left_child(id_r_r, 29);
        tree.add_right_child(id_r_r, 41);

        assert!(!is_bst(&tree));
    }
}
