struct Solution;

impl Solution {

    pub fn length_of_lis(nums: &[u32]) -> u32 {
        // We maintain an array `dominant` where dominant[k] holds the smallest possible
        // tail value of an increasing subsequence of length k+1.
        let mut dominant: Vec<u32> = Vec::with_capacity(nums.len());
        for num in nums {
            // for each value we search the number the LIS that we can extend: since dominant
            // is ordered we can use partition_point
            let p = dominant.partition_point(|&v| v < *num);
            if p == dominant.len() {
                // Extend the LIS
                dominant.push(*num);
            } else {
                // Improve the tail of a subsequence of length i+1 cause num is dominant in the sense that
                // num is smaller than the current tail of length p + 1
                dominant[p] = *num;
            }
        }
        // since we append a value only when we extend LIS of subproblems,
        // the length of the array corresponds to the length of the LIS for
        // the original problem
        dominant.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(4, Solution::length_of_lis(&[10,9,2,5,3,7,101,18]));
    }
}