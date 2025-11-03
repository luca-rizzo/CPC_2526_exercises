struct Solution;

impl Solution {
    // this solution has a complexity O(k + n) con k = right e n = #ranges;
    // this because we iterate over all ranges to increment left and decrement right extreme
    // and this has cost O(n) and then we compute a prefix sum of the array up to right and this
    // has cost O(k). Then we scan between left and right and this has cost O(k)
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut freq: Vec<i32> = vec![0; 52];
        for r in ranges {
            // this is a way to say from r[0] up to r[1] + 1 the range are covered
            // through the successive static prefix sum
            freq[r[0] as usize] += 1;
            freq[(r[1] + 1) as usize] -= 1;
        }
        let prefix_sum: Vec<_> = freq
            .iter()
            .take((right + 1) as usize)
            .scan(0, |acc: &mut i32, &val| {
                *acc += val;
                Some(*acc)
            })
            .collect();
        for i in left..right + 1 {
            if prefix_sum[i as usize] == 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    fn create_test_case(input: Vec<[i32; 2]>) -> Vec<Vec<i32>> {
        input.into_iter().map(|pair| pair.to_vec()).collect()
    }


    #[test]
    fn test_1() {
        let result = Solution::is_covered(create_test_case(vec![[1,2],[3,4],[5,6]]), 2, 5);
        assert!(result);
    }

    #[test]
    fn test_2() {
        let result = Solution::is_covered(create_test_case(vec![[1,10],[10,20]]), 21, 21);
        assert!(!result);
    }
}
