use std::cmp::{max, min};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Range {
    start: usize,
    end: usize,
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}..{}]", self.start, self.end)
    }
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Range { start, end }
    }

    pub fn contains(&self, other: Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn size(&self) -> usize {
        self.end - self.start + 1
    }

    pub fn intersect(&self, other: Range) -> Self {
        Range::new(max(self.start, other.start), min(self.end, other.end))
    }

    pub fn no_overlap(&self, other: Range) -> bool {
        self.end < other.start || other.end < self.start
    }

    pub fn middle(&self) -> usize {
        self.start + ((self.end - self.start) / 2)
    }
}

pub struct IntegerSegmentTree {
    tree: Vec<i32>,
    pending_updates: Vec<i32>,
    num_leaf: usize,
    neutral_value: i32,
    merge_values: fn(i32, i32) -> i32,
}

impl IntegerSegmentTree {
    pub fn get_index(&self, i: usize) -> i32 {
        self.tree[i]
    }

    pub fn build_empty(
        num_elems: usize,
        neutral_value: i32,
        merge_values: fn(i32, i32) -> i32,
    ) -> Self {
        let implicit_tree: Vec<i32> = vec![0; 2 * num_elems - 1];
        let segment_tree = IntegerSegmentTree {
            tree: implicit_tree,
            pending_updates: vec![0; 2 * num_elems - 1],
            num_leaf: num_elems,
            neutral_value,
            merge_values,
        };
        segment_tree
    }

    pub fn build(a: &[i32], neutral_value: i32, merge_values: fn(i32, i32) -> i32) -> Self {
        let mut implicit_tree: Vec<i32> = vec![0; 2 * a.len() - 1];
        Self::build_recursively(&a, &mut implicit_tree, 0, a.len() - 1, 0, merge_values);
        let segment_tree = IntegerSegmentTree {
            tree: implicit_tree,
            pending_updates: vec![0; 2 * a.len() - 1],
            num_leaf: a.len(),
            neutral_value,
            merge_values,
        };
        segment_tree
    }

    fn build_recursively(
        a: &[i32],
        tree: &mut Vec<i32>,
        l: usize,
        r: usize,
        index: usize,
        merge_values: fn(i32, i32) -> i32,
    ) {
        if l == r {
            tree[index] = a[l];
            return;
        }
        let middle = (l + r) / 2;
        let left_child_index = index + 1;
        let right_child_index = index + 2 * (middle - l + 1);
        Self::build_recursively(a, tree, l, middle, left_child_index, merge_values);
        Self::build_recursively(a, tree, middle + 1, r, right_child_index, merge_values);
        let value = merge_values(tree[left_child_index], tree[right_child_index]);
        tree[index] = value;
    }

    pub fn query(&mut self, i: usize, j: usize) -> i32 {
        if i > j {
            return self.neutral_value;
        }
        self.rec_query(Range::new(i, j), Range::new(0, self.num_leaf - 1), 0)
    }

    fn rec_query(&mut self, query_range: Range, node_segment: Range, index: usize) -> i32 {
        if node_segment.no_overlap(query_range) {
            return self.neutral_value;
        }
        let merge = self.merge_values;
        let tree = &mut self.tree;
        let pending = &mut self.pending_updates;
        if pending[index] > 0 {
            let pending_update = pending[index];
            let repeated_val = (1..node_segment.size()).fold(pending_update, |acc, _| {
                merge(acc, pending_update)
            });
            tree[index] = merge(tree[index], repeated_val);
            //propagate updates to his son
            let middle = node_segment.middle();
            let num_leaf_left_subtree = middle - node_segment.start + 1;
            let right_child_index = index + 2 * num_leaf_left_subtree;
            if node_segment.size() > 1 {
                println!(
                    "\t propagate update to left {} and right {} segment",
                    Range::new(node_segment.start, middle),
                    Range::new(middle + 1, node_segment.end)
                );
                pending[index + 1] =
                    merge(pending[index + 1], pending_update);
                pending[right_child_index] =
                    merge(pending[right_child_index], pending_update);
            }
            pending[index] = 0;
        }

        if query_range.contains(node_segment) {
            return tree[index];
        }

        // partial overlap
        let middle = node_segment.middle();
        let left_sum = self.rec_query(
            query_range,
            Range::new(node_segment.start, middle),
            index + 1,
        );
        let num_leaf_left_subtree = middle - node_segment.start + 1;
        // since the left subtree has num_leaf_left_subtree leaf it will have in total 2 * num_leaf_left_subtree - 1 nodes leaf included
        // so the index of the right child will be the next after this nodes
        let right_sum = self.rec_query(
            query_range,
            Range::new(middle + 1, node_segment.end),
            index + 2 * num_leaf_left_subtree,
        );
        merge(left_sum, right_sum)
    }

    pub fn add(&mut self, i: usize, val: i32) {
        self.add_rec(i, val, 0, self.num_leaf - 1, 0);
    }

    fn add_rec(&mut self, i: usize, val: i32, l_s: usize, r_s: usize, nav_index: usize) {
        if l_s == r_s {
            self.tree[nav_index] += val;
            return;
        }
        let middle = (l_s + r_s) / 2;
        let left_child_index = nav_index + 1;
        let right_child_index = nav_index + 2 * (middle - l_s + 1);
        if i <= middle {
            self.add_rec(i, val, l_s, middle, left_child_index);
        } else {
            self.add_rec(i, val, middle + 1, r_s, right_child_index);
        }
        self.tree[nav_index] =
            (self.merge_values)(self.tree[left_child_index], self.tree[right_child_index]);
    }

    fn range_update(&mut self, i: usize, j: usize, val: i32) {
        self.range_update_rec(Range::new(i, j), Range::new(0, self.num_leaf - 1), val, 0);
    }

    fn range_update_rec(
        &mut self,
        query_range: Range,
        node_segment: Range,
        val: i32,
        nav_index: usize,
    ) {
        if node_segment.no_overlap(query_range) {
            return;
        }
        let merge = self.merge_values;
        let tree = &mut self.tree;
        let pending = &mut self.pending_updates;
        if query_range.contains(node_segment) {
            //total overlap: node_segment is contained in query
            if node_segment.size() == 1 {
                // leaf: nodes that handles a segment with a single element
                tree[nav_index] = merge(tree[nav_index], val);
            } else {
                // save update as pending in order to be applied in successive query
                pending[nav_index] =
                    merge(val, pending[nav_index]);
            }
            return;
        }
        //partial overlap: query partially contained in node_segment
        let query_segment = query_range.intersect(node_segment);
        let repeated_val =
            (1..query_segment.size()).fold(val, |acc, _| merge(acc, val));

        tree[nav_index] = merge(repeated_val, tree[nav_index]);

        //navigate left and right
        let middle = node_segment.middle();
        self.range_update_rec(
            query_range,
            Range::new(node_segment.start, middle),
            val,
            nav_index + 1,
        );
        let num_leaf_left_subtree = middle - node_segment.start + 1;
        // since the left subtree has num_leaf_left_subtree leaf it will have in total 2 * num_leaf_left_subtree - 1 nodes leaf included
        // so the index of the right child will be the next after this nodes
        self.range_update_rec(
            query_range,
            Range::new(middle + 1, node_segment.end),
            val,
            nav_index + 2 * num_leaf_left_subtree,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::integer_segment_tree::IntegerSegmentTree;
    use std::cmp::{max, min};

    #[test]
    fn test_creation() {
        let result = IntegerSegmentTree::build(&[3, 4, 5, 3, 8, 12, -4], 0, |x, y| x + y);
        assert_eq!(31, result.get_index(0));
        assert_eq!(15, result.get_index(1));
        assert_eq!(16, result.get_index(8));
        assert_eq!(7, result.get_index(2));
        assert_eq!(8, result.get_index(5));
        assert_eq!(20, result.get_index(9));
        assert_eq!(3, result.get_index(3));
        assert_eq!(4, result.get_index(4));
        assert_eq!(5, result.get_index(6));
        assert_eq!(3, result.get_index(7));
        assert_eq!(8, result.get_index(10));
        assert_eq!(12, result.get_index(11));
        assert_eq!(-4, result.get_index(12));
    }

    #[test]
    fn test_query_even_n_even() {
        let mut result = IntegerSegmentTree::build(&[3, 4, 5, 3, 2], 0, |x, y| x + y);
        assert_eq!(9, result.query(1, 2));
    }

    #[test]
    fn test_query_even_n_odd() {
        let mut result = IntegerSegmentTree::build(&[3, 4, 5, 3, 2, 6, 2, 1], 0, |x, y| x + y);
        assert_eq!(16, result.query(2, 5));
    }

    #[test]
    fn test_query_odd_n_odd() {
        let mut result = IntegerSegmentTree::build(&[3, 4, 5, 3, 8, 12, -4], 0, |x, y| x + y);
        assert_eq!(16, result.query(4, 6));
    }

    #[test]
    fn test_query_odd_n_even() {
        let mut result = IntegerSegmentTree::build(&[3, 4, 5, 3, 8, 12, -4, 5], 0, |x, y| x + y);
        assert_eq!(8, result.query(2, 3));
    }

    #[test]
    fn test_min_query() {
        let mut result =
            IntegerSegmentTree::build(&[3, 4, 5, 3, 8, 12, -4, 5], i32::MAX, |x, y| min(x, y));
        assert_eq!(3, result.query(1, 4));
    }

    #[test]
    fn test_add() {
        let mut result = IntegerSegmentTree::build(&[3, 4, 5, 3, 8, 12, -4], 0, |x, y| x + y);
        result.add(4, -8);
        assert_eq!(23, result.get_index(0));
        assert_eq!(15, result.get_index(1));
        assert_eq!(8, result.get_index(8));
        assert_eq!(7, result.get_index(2));
        assert_eq!(8, result.get_index(5));
        assert_eq!(12, result.get_index(9));
        assert_eq!(3, result.get_index(3));
        assert_eq!(4, result.get_index(4));
        assert_eq!(5, result.get_index(6));
        assert_eq!(3, result.get_index(7));
        assert_eq!(0, result.get_index(10));
        assert_eq!(12, result.get_index(11));
        assert_eq!(-4, result.get_index(12));
    }

    #[test]
    fn test_multiple_add() {
        let mut result = IntegerSegmentTree::build_empty(10, 0, |x, y| x + y);
        result.add(0, 1);
        result.add(1, 1);
        result.add(2, 1);
        result.add(3, 1);
        result.add(4, 1);

        assert_eq!(1, result.query(4, 5));
    }

    #[test]
    fn test_range_update() {
        let mut result = IntegerSegmentTree::build_empty(10, 0, |x, y| x + y);
        result.range_update(0, 5, 5);

        assert_eq!(30, result.query(0, 5));
    }
    #[test]
    fn test_range_update_directly_child_of_lazy_node() {
        let mut result = IntegerSegmentTree::build_empty(10, 0, |x, y| x + y);
        result.range_update(0, 5, 5);

        assert_eq!(15, result.query(0, 2));
    }
    #[test]
    fn test_range_update_directly_leaf() {
        let mut result = IntegerSegmentTree::build_empty(10, 0, |x, y| x + y);
        result.range_update(0, 5, 5);

        assert_eq!(5, result.query(4, 4));
    }

    #[test]
    fn test_range_update_propagation_until_leaf() {
        let mut result = IntegerSegmentTree::build_empty(10, 0, |x, y| x + y);
        result.range_update(0, 5, 5);

        assert_eq!(30, result.query(0, 5));
        assert_eq!(15, result.query(0, 2));
        assert_eq!(10, result.query(0, 1));
        assert_eq!(5, result.query(1, 1));
        assert_eq!(5, result.query(0, 0));
        assert_eq!(5, result.query(2, 2));
    }

    #[test]
    fn test_range_update_max() {
        let mut result = IntegerSegmentTree::build_empty(10, 0, |x, y| max(x, y));
        result.range_update(0, 5, 10);

        //[10, 10, 10, 10, 10, 10, 0, 0, 0, 0]
        assert_eq!(10, result.query(0, 5));
    }

    #[test]
    fn test_range_update_max_directly_child_of_lazy_node() {
        let mut result = IntegerSegmentTree::build_empty(10, 0, |x, y| max(x, y));
        result.range_update(0, 5, 10);

        //[10, 10, 10, 10, 10, 10, 0, 0, 0, 0]
        assert_eq!(10, result.query(0, 2));
    }

    #[test]
    fn test_range_update_max_directly_leaf_of_to() {
        let mut result = IntegerSegmentTree::build_empty(10, 0, |x, y| max(x, y));
        result.range_update(0, 5, 10);

        //[10, 10, 10, 10, 10, 10, 0, 0, 0, 0]
        assert_eq!(10, result.query(4, 4));
        assert_eq!(10, result.query(3, 3));
    }

    #[test]
    fn test_range_update_existing_elem() {
        let mut result = IntegerSegmentTree::build(&[10, 11, 34, 2, 23], 0, |x, y| max(x, y));
        assert_eq!(34, result.query(0, 4));

        result.range_update(3, 4, 45);

        assert_eq!(45, result.query(0, 4));
        assert_eq!(34, result.query(0, 2));
    }

    #[test]
    fn test_multiple_range_update() {
        let mut result = IntegerSegmentTree::build_empty(10, 0, |x, y| max(x, y));
        result.range_update(0, 5, 10);
        result.range_update(2, 5, 20);

        //[10, 10, 10, 20, 20, 20, 0, 0, 0, 0]
        assert_eq!(20, result.query(4, 4));
        assert_eq!(10, result.query(1, 1));
    }

    #[test]
    fn test_multiple_range_update_first_greater() {
        let mut result = IntegerSegmentTree::build_empty(10, 0, |x, y| max(x, y));
        result.range_update(0, 5, 20);
        result.range_update(2, 5, 10);

        //[10, 10, 10, 20, 20, 20, 0, 0, 0, 0]
        assert_eq!(20, result.query(4, 4));
        assert_eq!(20, result.query(1, 1));
    }
}
