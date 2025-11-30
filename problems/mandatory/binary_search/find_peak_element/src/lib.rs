struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let length = nums.len();
        if length == 1 {
            return 0;
        }
        let mut low = 0;
        let mut high = length - 1;
        while low <= high {
            let middle = low + (high + low) / 2;
            let p_middle = if middle == 0 {
                i32::MIN
            } else {
                nums[middle - 1]
            };
            let n_middle = if middle == length - 1 {
                i32::MIN
            } else {
                nums[middle + 1]
            };
            if (nums[middle] > p_middle) & (nums[middle] > n_middle) {
                return middle as i32;
            } else if nums[middle] > n_middle {
                high = middle - 1;
            } else {
                low = middle + 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;


}
