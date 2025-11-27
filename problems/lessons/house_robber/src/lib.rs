use graph::{Dag, maximum_path_beneficial_from_to, minimum_paths_cost_from_to};
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut prev_1, mut prev_2) = (0, 0);
        for revenue in nums {
            let temp = max(prev_1, revenue + prev_2);
            prev_2 = prev_1;
            prev_1 = temp;
        }
        prev_1
    }

    pub fn rob_with_dag(nums: Vec<i32>) -> i32 {
        let num_nodes = nums.len() + 2;
        let mut dag = Dag::new(num_nodes);

        // archi da start e archi a end
        for i in 0..nums.len() {
            let node_idx = i + 1;
            dag.add_edge(0, node_idx, nums[i] as u32); // start → casa
            dag.add_edge(node_idx, nums.len() + 1, 0); // casa → end
            if i + 2 < nums.len() {
                dag.add_edge(node_idx, node_idx + 2, nums[i + 2] as u32);
            }
            if i + 3 < nums.len() {
                dag.add_edge(node_idx, node_idx + 3, nums[i + 3] as u32);
            }
        }

        maximum_path_beneficial_from_to(&dag, 0, num_nodes - 1)
            .unwrap()
            .1 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(4, Solution::rob_with_dag(vec![1, 2, 3, 1]));
    }
}
