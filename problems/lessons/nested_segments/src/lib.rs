use segment_tree::IntegerSegmentTree;
use std::collections::HashMap;
struct Solution;

impl Solution {

    pub fn nested_segment(segments: &[(i32, i32)]) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; segments.len()];
        // since we need a counter of the extreme values of the segments, we need to reduce the range of values
        // of the extremes, so we remap all the extreme segments to their rank in the sorted array
        let (mut remapped, n_extremes) = Self::remap_segments(segments);
        // create a segment tree to count frequency of all possible extreme of remapped segments
        let mut segment_tree: IntegerSegmentTree =
            IntegerSegmentTree::build_empty(n_extremes, 0, |x, y| x + y);
        for &(s, _, _) in &remapped {
            segment_tree.add(s, 1);
        }
        // start from "last segment", ie the one that ends last
        remapped.sort_by(|a, b| b.1.cmp(&a.1));

        for &(s, e, original_index) in &remapped {
            // this counts all segments starting between s+1 and e-1, so the segment starting within the
            // last segment. Since this is the final segment, all these segments end before
            // this segment, so they are included
            res[original_index] = segment_tree.query(s + 1, e - 1);
            // remove contribution last segment in order to create a new one "last segment"
            segment_tree.add(s, -1);
        }

        res
    }

    fn remap_segments(segments: &[(i32, i32)]) -> (Vec<(usize, usize, usize)>, usize) {
        let mut extreme: Vec<i32> = segments.iter().flat_map(|&(s, e)| [s, e]).collect();
        extreme.sort_unstable();
        extreme.dedup();
        let pos: HashMap<i32, usize> = extreme.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        // remap
        (
            segments
                .iter()
                .enumerate()
                .map(|(index, (s, e))| (pos[s], pos[e], index))
                .collect(),
            extreme.len(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1_from_statement() {
        let segments = vec![(1, 8), (2, 3), (4, 7), (5, 6)];
        let ans = Solution::nested_segment(&segments);
        assert_eq!(ans, vec![3, 0, 1, 0]);
    }

    #[test]
    fn sample_2_from_statement() {
        let segments = vec![(3, 4), (1, 5), (2, 6)];
        let ans = Solution::nested_segment(&segments);
        assert_eq!(ans, vec![0, 1, 1]);
    }

    #[test]
    fn all_disjoint() {
        let segments = vec![(1, 2), (3, 4), (5, 6), (7, 8)];
        let ans = Solution::nested_segment(&segments);
        assert_eq!(ans, vec![0, 0, 0, 0]);
    }

    #[test]
    fn chain_nesting() {
        let segments = vec![(1, 10), (2, 9), (3, 8), (4, 7), (5, 6)];
        let ans = Solution::nested_segment(&segments);
        assert_eq!(ans, vec![4, 3, 2, 1, 0]);
    }

    #[test]
    fn mixed_signs_big_coords() {
        let segments = vec![(-10, 10), (-5, 0), (-4, -1), (-3, -2), (5, 9)];
        let ans = Solution::nested_segment(&segments);
        assert_eq!(ans, vec![4, 2, 1, 0, 0]);
    }

    #[test]
    fn single_segment() {
        let segments = vec![(42, 100)];
        let ans = Solution::nested_segment(&segments);
        assert_eq!(ans, vec![0]);
    }
}
