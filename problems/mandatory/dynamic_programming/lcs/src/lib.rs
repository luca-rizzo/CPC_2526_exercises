use std::cmp::max;
use std::mem;

struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (shorter, longer) = if text1.len() > text2.len() {
            (text2, text1)
        } else {
            (text1, text2)
        };
        let shorter: Vec<char> = shorter.chars().collect();
        let longer: Vec<char> = longer.chars().collect();
        let mut curr_r = vec![0; shorter.len() + 1];
        let mut prev_r = vec![0; shorter.len() + 1];
        for (_, r) in longer.iter().enumerate() {
            for (c_index, c) in shorter.iter().enumerate() {
                let c_index = c_index + 1;
                if c == r {
                    curr_r[c_index] = prev_r[c_index - 1] + 1;
                } else {
                    curr_r[c_index] = max(curr_r[c_index - 1], prev_r[c_index]);
                }
            }
            mem::swap(&mut curr_r, &mut prev_r);
        }
        *prev_r.last().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_3() {
        assert_eq!(
            3,
            Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string())
        );
    }
}
