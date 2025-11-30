use std::mem;

struct Solution;


impl Solution {
    /* Dynamic Programming Problem
       We create a table where T[i, j] = true if, considering the first i elements,
       we are able to obtain a sum equal to j. The table has size n × (sum/2)
       because we want to know whether, using all n elements, we can reach the sum
       sum/2, which is necessary for the required partition.

       Subproblems:
           These are the cases in which we have k < i numbers and a sum l < j.

       Merge step:
           Assuming we know the answers for the subproblems, how do we combine them
           to obtain the answer for the current problem T[i, j]? For each number we
           can decide whether to take it or not:
             1) If we do NOT take it, we can reach sum j only if the previous i−1
                elements already allowed us to reach sum j
                -> T[i, j] = T[i−1, j]

             2) If we DO take it, we can reach sum j only if the previous i−1
                elements allowed us to reach sum j − nums[i]
                -> T[i, j] = T[i−1, j − nums[i]]

           Therefore:
               T[i, j] = T[i−1, j] || T[i−1, j − nums[i]]
    */
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let overall_sum: i32 = nums.iter().sum();
        if overall_sum % 2 != 0 {
            return false;
        }
        let cols = (overall_sum / 2) as usize + 1usize;
        let rows = nums.len() + 1;
        let mut mat = vec![vec![false; cols]; rows];
        mat[0][0] = true;
        for i in 1..rows {
            let num = nums[i-1] as usize;
            let (before, after) = mat.split_at_mut(i);
            after[0].copy_from_slice(&before[i - 1]);
            for j in num..cols {
                mat[i][j] |= mat[i-1][j - num];
            }
        }
        mat[rows - 1][cols - 1]
    }

    /*
        Optimized version with only 2 rows that keep track of current and previous row
    */

    pub fn can_partition_2_rows(nums: Vec<i32>) -> bool {
        let overall_sum: i32 = nums.iter().sum();
        if overall_sum % 2 != 0 {
            return false;
        }
        let cols = (overall_sum / 2) as usize + 1usize;
        let rows = nums.len() + 1;
        let mut c_r = vec![false; cols];
        let mut p_r = vec![false; cols];
        p_r[0] = true;
        for num in nums {
            let num = num as usize;
            for j in num..cols {
                c_r[j] = p_r[j] || p_r[j - num];
            }
            p_r = c_r.clone();
        }
        p_r[cols - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert!(Solution::can_partition(vec![1,5,11,5]));
        assert!(Solution::can_partition_2_rows(vec![1,5,11,5]));
    }
    #[test]
    fn test_case_2() {
        assert!(!Solution::can_partition(vec![1,2,3,5]));
        assert!(!Solution::can_partition_2_rows(vec![1,2,3,5]));
    }

}