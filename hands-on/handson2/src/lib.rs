#![allow(unused)]

pub mod min_max {
    use std::cmp::{max, min};
    use std::error::Error;
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

        pub fn total_overlap(&self, other: Range) -> bool {
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

    struct MinMaxArray {
        st: SegmentTree,
    }

    impl MinMaxArray {
        pub fn build(a: &[u32]) -> Self {
            Self {
                st: SegmentTree::build(a),
            }
        }

        pub fn max(&mut self, i: usize, j: usize) -> u32 {
            self.st.query(i, j)
        }

        pub fn update(&mut self, i: usize, j: usize, t: u32) {
            self.st.range_update(i, j, t);
        }
    }

    struct SegmentTree {
        tree: Vec<u32>,
        pending_updates: Vec<u32>,
        num_leaf: usize,
    }

    impl SegmentTree {
        pub fn build_empty(num_elems: usize) -> Self {
            let implicit_tree: Vec<u32> = vec![u32::MAX; 2 * num_elems - 1];
            let segment_tree = Self {
                tree: implicit_tree,
                pending_updates: vec![u32::MAX; 2 * num_elems - 1],
                num_leaf: num_elems,
            };
            segment_tree
        }

        pub fn build(a: &[u32]) -> Self {
            let mut implicit_tree: Vec<u32> = vec![u32::MAX; 2 * a.len() - 1];
            Self::build_rec(&a, &mut implicit_tree, Range::new(0, a.len() - 1), 0);
            let segment_tree = Self {
                tree: implicit_tree,
                pending_updates: vec![u32::MAX; 2 * a.len() - 1],
                num_leaf: a.len(),
            };
            segment_tree
        }

        fn build_rec(a: &[u32], tree: &mut Vec<u32>, node_segment: Range, index: usize) {
            if node_segment.size() == 1 {
                tree[index] = a[node_segment.start];
                return;
            }
            let middle = node_segment.middle();
            let left_child_index = index + 1;
            let right_child_index = index + 2 * (middle - node_segment.start + 1);
            Self::build_rec(
                a,
                tree,
                Range::new(node_segment.start, middle),
                left_child_index,
            );
            Self::build_rec(
                a,
                tree,
                Range::new(middle + 1, node_segment.end),
                right_child_index,
            );
            tree[index] = max(tree[left_child_index], tree[right_child_index]);
        }

        pub fn query(&mut self, i: usize, j: usize) -> u32 {
            if i > j {
                return u32::MIN;
            }
            self.query_rec(Range::new(i, j), Range::new(0, self.num_leaf - 1), 0)
        }

        fn query_rec(&mut self, query_range: Range, node_segment: Range, index: usize) -> u32 {
            self.handle_pending_updates(node_segment, index);
            if query_range.no_overlap(node_segment) {
                return u32::MIN;
            }
            if query_range.total_overlap(node_segment) {
                return self.tree[index];
            }
            // partial overlap
            let middle = node_segment.middle();
            let left_value = self.query_rec(
                query_range,
                Range::new(node_segment.start, middle),
                index + 1,
            );
            let num_leaf_left_subtree = middle - node_segment.start + 1;
            // since the left subtree has num_leaf_left_subtree leaf it will have in total 2 * num_leaf_left_subtree - 1 nodes leaf included
            // so the index of the right child will be the next after this nodes
            let right_value = self.query_rec(
                query_range,
                Range::new(middle + 1, node_segment.end),
                index + 2 * num_leaf_left_subtree,
            );
            max(left_value, right_value)
        }

        fn handle_pending_updates(&mut self, node_segment: Range, index: usize) {
            if self.pending_updates[index] != u32::MAX {
                let pending_update = self.pending_updates[index];
                self.update_node_and_propagate(node_segment, index, pending_update);
                self.pending_updates[index] = u32::MAX;
            }
        }

        fn update_node_and_propagate(&mut self, node_segment: Range, index: usize, to_update: u32) {
            self.tree[index] = min(self.tree[index], to_update);
            if node_segment.size() > 1 {
                //propagate updates to his children
                let middle = node_segment.middle();
                let num_leaf_left_subtree = middle - node_segment.start + 1;
                let right_child_index = index + 2 * num_leaf_left_subtree;
                self.pending_updates[index + 1] = min(self.pending_updates[index + 1], to_update);
                self.pending_updates[right_child_index] =
                    min(self.pending_updates[right_child_index], to_update);
            }
        }

        fn range_update(&mut self, i: usize, j: usize, val: u32) {
            self.range_update_rec(Range::new(i, j), Range::new(0, self.num_leaf - 1), val, 0);
        }

        fn range_update_rec(
            &mut self,
            query_range: Range,
            node_segment: Range,
            val: u32,
            nav_index: usize,
        ) {
            self.handle_pending_updates(node_segment, nav_index);
            if node_segment.no_overlap(query_range) {
                return;
            }
            if query_range.total_overlap(node_segment) {
                //total overlap: node_segment is contained in query
                self.update_node_and_propagate(node_segment, nav_index, val);
                return;
            }
            //partial overlap: query partially contained in node_segment
            //navigate left and right
            let middle = node_segment.middle();
            let left_child_index = nav_index + 1;
            self.range_update_rec(
                query_range,
                Range::new(node_segment.start, middle),
                val,
                left_child_index,
            );
            // since the left subtree has num_leaf_left_subtree leaf it will have in total 2 * num_leaf_left_subtree - 1 nodes leaf included
            // so the index of the right child will be the next after this nodes
            let right_child_index = nav_index + 2 * (middle - node_segment.start + 1);
            self.range_update_rec(
                query_range,
                Range::new(middle + 1, node_segment.end),
                val,
                right_child_index,
            );
            self.tree[nav_index] = max(self.tree[left_child_index], self.tree[right_child_index]);
        }
    }

    pub fn solve(input: &String) -> Result<String, Box<dyn Error>> {
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().ok_or("missing n")?.parse()?;
        let m: usize = iter.next().ok_or("missing m")?.parse()?;
        let mut arr: Vec<u32> = (0..n)
            .map(|_| -> Result<u32, Box<dyn Error>> {
                let s = iter.next().ok_or("missing value")?;
                Ok(s.parse::<u32>()?)
            })
            .collect::<Result<_, _>>()?;

        let mut st = SegmentTree::build(arr.as_slice());
        let mut output = String::new();

        for _ in 0..m {
            let query_type: u8 = iter.next().ok_or("missing query type")?.parse()?;
            if query_type == 0 {
                // Update(i, j, T)
                let l: usize = iter.next().ok_or("missing l")?.parse()?;
                let r: usize = iter.next().ok_or("missing r")?.parse()?;
                let t: u32 = iter.next().ok_or("missing val")?.parse()?;
                st.range_update(l - 1, r - 1, t);
            } else if query_type == 1 {
                // Max(i, j)
                let l: usize = iter.next().ok_or("missing l")?.parse()?;
                let r: usize = iter.next().ok_or("missing r")?.parse()?;
                let ans = st.query(l - 1, r - 1);
                output.push_str(&format!("{}\n", ans));
            } else {
                output.push_str(&"unknown query type\n".to_string());
            }
        }

        Ok(output)
    }

    #[cfg(test)]
    mod segment_tree_tests {
        use crate::min_max::SegmentTree;

        #[test]
        fn test_small_array_build() {
            let mut tree = SegmentTree::build(&[7, 10, 2, 21]);

            assert_eq!(21, tree.query(0, 3));
            assert_eq!(10, tree.query(0, 1));
            assert_eq!(21, tree.query(2, 3));
            assert_eq!(7, tree.query(0, 0));
            assert_eq!(10, tree.query(1, 1));
            assert_eq!(2, tree.query(2, 2));
            assert_eq!(21, tree.query(3, 3));
        }

        #[test]
        fn test_medium_array_build() {
            let mut tree = SegmentTree::build(&[8, 2, 6, 21, 18, 15, 2, 31, 4, 16]);

            // Leaf
            assert_eq!(8, tree.query(0, 0));
            assert_eq!(2, tree.query(1, 1));
            assert_eq!(6, tree.query(2, 2));
            assert_eq!(21, tree.query(3, 3));
            assert_eq!(18, tree.query(4, 4));
            assert_eq!(15, tree.query(5, 5));
            assert_eq!(2, tree.query(6, 6));
            assert_eq!(31, tree.query(7, 7));
            assert_eq!(4, tree.query(8, 8));
            assert_eq!(16, tree.query(9, 9));

            assert_eq!(8, tree.query(0, 1));
            assert_eq!(6, tree.query(1, 2));
            assert_eq!(21, tree.query(3, 4));
            assert_eq!(15, tree.query(5, 6));
            assert_eq!(31, tree.query(6, 7));
            assert_eq!(16, tree.query(8, 9));

            assert_eq!(8, tree.query(0, 2));
            assert_eq!(21, tree.query(0, 3));
            assert_eq!(21, tree.query(0, 4));
            assert_eq!(21, tree.query(3, 4));
            assert_eq!(31, tree.query(5, 7));
            assert_eq!(16, tree.query(8, 9));
            assert_eq!(31, tree.query(5, 9));

            assert_eq!(31, tree.query(0, 9));
            assert_eq!(31, tree.query(2, 9));
            assert_eq!(31, tree.query(4, 9));
            assert_eq!(31, tree.query(7, 9));
        }

        #[test]
        fn hands_on_example() {
            // 5 3       // n m
            // 5 1 4 3 2 // The array A
            // 0 1 2 2   // Update(1, 2, 2). The array A becomes 2 1 4 3 2.
            // 1 2 4     // Max(2, 4) = 4
            // 1 1 2     // Max(1, 2) = 2

            let mut tree = SegmentTree::build(&[5, 1, 4, 3, 2]);

            tree.range_update(0, 1, 2);
            assert_eq!(4, tree.query(1, 3));
            assert_eq!(2, tree.query(0, 1));
        }

        #[test]
        fn test_range_update_right_child_root() {
            let mut tree = SegmentTree::build(&[8, 2, 6, 21, 18, 15, 2, 31, 4, 16]);

            tree.range_update(5, 9, 2);
            assert_eq!(21, tree.query(0, 9));
        }

        #[test]
        fn test_range_update_leaf() {
            let mut tree = SegmentTree::build(&[8, 2, 6, 21, 18, 15, 2, 31, 4, 16]);

            tree.range_update(7, 7, 2);
            assert_eq!(21, tree.query(0, 9));
        }

        #[test]
        fn test_range_update_propagation_until_leaf() {
            let mut tree = SegmentTree::build(&[8, 2, 6, 21, 18, 15, 2, 31, 4, 16]);

            tree.range_update(4, 9, 2);
            assert_eq!(2, tree.query(5, 7));
            assert_eq!(2, tree.query(5, 6));
            assert_eq!(2, tree.query(5, 5));
            assert_eq!(2, tree.query(6, 6));
        }

        #[test]
        fn test_range_update_internal_propagation_until_leaf() {
            let mut tree = SegmentTree::build(&[8, 2, 6, 21, 18, 15, 2, 31, 4, 16]);

            tree.range_update(4, 9, 2);
            //[5,9] updated
            assert_eq!(2, tree.tree[10]);
            assert_eq!(u32::MAX, tree.pending_updates[10]);
            //[5, 7] still not updated but lazy update recorded
            assert_eq!(31, tree.tree[11]);
            assert_eq!(2, tree.pending_updates[11]);
            //[8,9] still not updated but lazy update recorded
            assert_eq!(16, tree.tree[16]);
            assert_eq!(2, tree.pending_updates[16]);

            assert_eq!(2, tree.query(5, 7));
            //[5,7] updated
            assert_eq!(2, tree.tree[11]);
            assert_eq!(u32::MAX, tree.pending_updates[11]);
            //[5, 6] still not updated but lazy update recorded
            assert_eq!(15, tree.tree[12]);
            assert_eq!(2, tree.pending_updates[12]);
            //[7, 7] still not updated but lazy update recorded
            assert_eq!(31, tree.tree[15]);
            assert_eq!(2, tree.pending_updates[15]);

            assert_eq!(2, tree.query(5, 6));
            //[5,6] updated
            assert_eq!(2, tree.tree[12]);
            assert_eq!(u32::MAX, tree.pending_updates[12]);
            //[6, 6] still not updated but lazy update recorded
            assert_eq!(15, tree.tree[13]);
            assert_eq!(2, tree.pending_updates[13]);
            //[7, 7] still not updated but lazy update recorded
            assert_eq!(2, tree.tree[14]);
            assert_eq!(2, tree.pending_updates[14]);

            assert_eq!(2, tree.query(8, 9));
            //[8, 9] updated
            assert_eq!(2, tree.tree[16]);
            assert_eq!(u32::MAX, tree.pending_updates[16]);
            //[8, 8] still not updated but lazy update recorded
            assert_eq!(4, tree.tree[17]);
            assert_eq!(2, tree.pending_updates[17]);
            //[9, 9] still not updated but lazy update recorded
            assert_eq!(16, tree.tree[18]);
            assert_eq!(2, tree.pending_updates[18]);

            assert_eq!(2, tree.query(8, 8));

            assert_eq!(2, tree.query(9, 9));
        }

        #[test]
        fn test_range_update_directly_leaf_of_lazy_update() {
            let mut tree = SegmentTree::build(&[8, 2, 6, 21, 18, 15, 2, 31, 4, 16]);

            tree.range_update(4, 9, 2);
            assert_eq!(2, tree.query(6, 6));
            assert_eq!(2, tree.query(5, 5));
        }
    }

    #[cfg(test)]
    mod hands_on_tests {
        use crate::min_max::solve;

        fn run_case(i: usize) -> Result<(), Box<dyn std::error::Error>> {
            let input = std::fs::read_to_string(format!("./test_set/input{i}.txt"))?;
            let expected = std::fs::read_to_string(format!("./test_set/output{i}.txt"))?;
            let actual = solve(&input)?;
            assert_eq!(actual, expected, "Mismatch on case {i}");
            Ok(())
        }

        #[test]
        fn case0() -> Result<(), Box<dyn std::error::Error>> {
            run_case(0)
        }
        #[test]
        fn case1() -> Result<(), Box<dyn std::error::Error>> {
            run_case(1)
        }
        #[test]
        fn case2() -> Result<(), Box<dyn std::error::Error>> {
            run_case(2)
        }
        #[test]
        fn case3() -> Result<(), Box<dyn std::error::Error>> {
            run_case(3)
        }
        #[test]
        fn case4() -> Result<(), Box<dyn std::error::Error>> {
            run_case(4)
        }
        #[test]
        fn case5() -> Result<(), Box<dyn std::error::Error>> {
            run_case(5)
        }
        #[test]
        fn case6() -> Result<(), Box<dyn std::error::Error>> {
            run_case(6)
        }
        #[test]
        fn case7() -> Result<(), Box<dyn std::error::Error>> {
            run_case(7)
        }
        #[test]
        fn case8() -> Result<(), Box<dyn std::error::Error>> {
            run_case(8)
        }
        #[test]
        fn case9() -> Result<(), Box<dyn std::error::Error>> {
            run_case(9)
        }
        #[test]
        fn case10() -> Result<(), Box<dyn std::error::Error>> {
            run_case(10)
        }
    }
}
