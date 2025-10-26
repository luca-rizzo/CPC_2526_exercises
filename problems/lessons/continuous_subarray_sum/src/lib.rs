use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        // Math background: if two number x and y, with y > x  has the same value % k that means that their difference is divisible by k.
        // In fact if x % k = i and y % k = i that means that x = zk +i and y = vk + i for some z and v in N.
        // So
        //  y - x = zk +i - vk - i = (z-v)k
        // and (z-v)*k is multiple of k
        // Application: if two prefix sum P[i] and P[j] has the same remainder it means that the difference P[j] - P[i] is divisible by k BUT this
        // difference is the **rangeSum** between the entry A[i + 1] and A[j] (the sum of a consecutive portion of nums)
        // For this reason we use a hash set to store the rangeSum values obtained in order to check if we already has seen the same value % k
        // and return true in this case
        let mut hash_map: HashMap<i32, i32> = HashMap::new();
        let mut remainder = 0;
        for (idx, &n) in nums.iter().enumerate() {
            // remember that (remainder + n) % k == (prefix_sum + n) % k
            remainder = (remainder + n) % k;
            if idx > 0 && remainder == 0 {
                return true;
            }
            if idx > 0 && remainder == 0 {
                return true;
            }
            if let Some(&prev_idx) = hash_map.get(&remainder) {
                if idx as i32 - prev_idx > 1 {
                    return true;
                }
            } else {
                hash_map.insert(remainder, idx as i32);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 7));
    }

    #[test]
    fn test_2() {
        assert!(Solution::check_subarray_sum(vec![5, 0, 0, 0], 3));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::check_subarray_sum(vec![1, 0], 2));
    }
}
