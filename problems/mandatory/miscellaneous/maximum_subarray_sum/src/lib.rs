use std::cmp::max;

struct Solution;

impl Solution {

    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let n = nums.len();
        let mut max_sum = nums[0];
        let mut sub_sum = nums[0];
        for start_idx in 1..n {
            if sub_sum > 0 {
                sub_sum += nums[start_idx];
            } else {
                sub_sum = nums[start_idx];
            }
            /* without if-else
                sub_sum = nums[start_idx].max(sub_sum + nums[start_idx]);           
             */
            max_sum = max(sub_sum, max_sum);
        }
        max_sum
    }

    pub fn brute_force_max_sub_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_sum = i32::MIN;
        for i in 0..n {
            let mut sum = 0;
            for j in i..n {
                sum += nums[j];
                max_sum = max_sum.max(sum);
            }
        }
        max_sum
    }

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_2() {
        let result = Solution::max_sub_array(vec![1]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_3() {
        let result = Solution::max_sub_array(vec![5,4,-1,7,8]);
        assert_eq!(result, 23);
    }

}
