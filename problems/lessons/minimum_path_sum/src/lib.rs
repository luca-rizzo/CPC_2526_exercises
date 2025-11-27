use graph::{Dag, minimum_paths_cost_from_to};
use std::cmp::min;
use std::mem;

struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 0 {
            return 0;
        }
        let n_cols = grid[0].len();
        let n_rows = grid.len();
        let mut curr_row = vec![i32::MAX; n_cols];
        let mut prev_row = vec![i32::MAX; n_cols];
        for r_index in 0..n_rows {
            for c_index in 0..n_cols {
                let cell_value = grid[r_index][c_index];
                if c_index == 0 && r_index == 0 {
                    curr_row[c_index] = cell_value;
                } else if c_index == 0 {
                    curr_row[c_index] = prev_row[c_index] + cell_value;
                } else if r_index == 0 {
                    curr_row[c_index] = curr_row[c_index - 1] + cell_value;
                } else {
                    let from_left = curr_row[c_index - 1];
                    let from_up = prev_row[c_index];
                    curr_row[c_index] = min(from_left, from_up) + cell_value;
                }
            }
            mem::swap(&mut prev_row, &mut curr_row);
        }
        *prev_row.last().unwrap_or(&i32::MAX)
    }

    pub fn min_path_sum_dag(grid: Vec<Vec<i32>>) -> i32 {
        let n_rows = grid.len();
        let n_cols = grid[0].len();
        let num_nodes = n_rows * n_cols + 1;
        let mut dag = Dag::new(num_nodes);
        for r_index in 0..n_rows {
            for c_index in 0..n_cols {
                let node_idx = r_index * n_cols + c_index + 1;
                // right
                if c_index != n_cols - 1 {
                    dag.add_edge(node_idx, node_idx + 1, grid[r_index][c_index + 1] as u32);
                }
                // down
                if r_index != n_rows - 1 {
                    let down_node_idx = (r_index + 1) * n_cols + c_index + 1;
                    dag.add_edge(node_idx, down_node_idx, grid[r_index + 1][c_index] as u32);
                }
            }
        }

        dag.add_edge(0, 1, grid[0][0] as u32);
        minimum_paths_cost_from_to(&dag, 0, num_nodes - 1)
            .unwrap()
            .1 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            7,
            Solution::min_path_sum_dag(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]])
        );
        assert_eq!(
            7,
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]])
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            12,
            Solution::min_path_sum_dag(vec![vec![1, 2, 3], vec![4, 5, 6]])
        );
        assert_eq!(
            12,
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]])
        );
    }
}
