use std::cmp::{max, min};

struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let max_with_acc = |acc: &mut i32, &val| {
            *acc = max(val, *acc);
            Some(*acc)
        };
        let left_max_prefix: Vec<i32> = height.iter().scan(0, max_with_acc).collect();
        let mut right_max_prefix: Vec<i32> = height.iter().rev().scan(0, max_with_acc).collect();
        right_max_prefix.reverse();

        height.iter().enumerate().fold(0, |acc, (i, &val)| {
            let left = if i > 0 { left_max_prefix[i - 1] } else { 0 };
            let right = if i < height.len() - 1 {
                right_max_prefix[i + 1]
            } else {
                0
            };
            let water_above = max(0, min(left, right) - val);
            acc + water_above
        })
    }

    pub fn trap_no_extra_space(height: Vec<i32>) -> i32 {
        // the idea is to use two pointer and for each iteration to calculate water and advance the pointer for
        // which we can SECURE calculate the water above using left or right maximum seen so far since this limits
        // the quantity of water that can stay above him
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut left_max = height[l];
        let mut right_max = height[r];
        let mut acc = 0;
        while r!= 0  && r >= l {
            if right_max > left_max {
                // this is the case in which we can compute the water above height[l] because for sure
                // this quantity is limited by left_max and allowed by right_max
                //  left_max = 2  l        right_max = 3
                acc += max(0, left_max - height[l]);
                left_max = max(left_max, height[l]);
                l += 1;
            } else {
                // this is the case in which we can compute the water above height[r] because for sure
                // this quantity is limited by right_max and allowed by left_max
                //  left_max = 3           r   right_max = 2
                acc += max(0, right_max - height[r]);
                right_max = max(right_max, height[r]);
                r -= 1;
            }
        }
        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap_no_extra_space(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap_no_extra_space(vec![0]), 0);
    }
}
