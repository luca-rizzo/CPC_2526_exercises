struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // We maintain an array `dominant` where dominant[k] holds the smallest possible
        // tail value of an increasing subsequence of length k+1.
        let mut dominant: Vec<Option<i32>> = vec![None; nums.len()];
        for num in nums {
            // for each value we search the number the LIS that we can extend
            let p = dominant.partition_point(|v| match v {
                Some(x) => *x < num,
                None => false,
            });
            // if num is dominant in the sense that num is smaller than the current tail of
            // the is of lenght k + 1
            if dominant[p].is_none() || dominant[p].unwrap() > num {
                dominant[p] = Some(num);
            }
        }
        dominant.iter()
            .rposition(|v| v.is_some())
            .map(|x| x + 1)
            .unwrap_or(0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(4, Solution::length_of_lis(vec![10,9,2,5,3,7,101,18]));
    }
}