struct FirstSolution;

//all the solution must be O(1) in space
impl FirstSolution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let n = (nums.len() as u32) - 1;
        // bits needed to represent n
        let number_of_bits = 32 - n.leading_zeros();

        let mut prefix_matched: u32 = 0;

        for i in (0..number_of_bits).rev() {
            let mut c_0 = 0u32;
            let mut c_1 = 0u32;

            for v in nums.iter() {
                // quanti bit del prefisso sono già fissati
                let bits_p = number_of_bits - i - 1;
                if Self::match_prefix(v, &prefix_matched, bits_p, number_of_bits) {
                    match Self::get_bit(v, &i) {
                        0 => c_0 += 1,
                        1 => c_1 += 1,
                        _ => {}
                    }
                }
            }

            // how many 1 and 0 there are as next bit between base and block less than n ?
            // starting from base (if the prefix is 11 and number of bits = 4 -> base = 1100 and block = 100)
            // the next bit divide the interval [base, base + block - 1] in 2
            // next bit 0: [base, base + half - 1]
            // next bit 1: [base + half, base + block - 1]
            let bits_left = i + 1;
            let base = prefix_matched << bits_left;
            let block = 1u32 << bits_left;
            let half = block >> 1;

            // We then clamp these intervals to 1..=n to count how many numbers
            // *should* exist in each half in the ideal range without duplicates
            let e0 = Self::clamp_count(base, base + half - 1, n);
            let e1 = Self::clamp_count(base + half, base + block - 1, n);

            // you choose the bit with more difference
            let d0 = c_0 as i32 - e0 as i32;
            let d1 = c_1 as i32 - e1 as i32;
            let next_bit = if d1 > d0 { 1 } else { 0 };

            // append of the bit
            prefix_matched = Self::append_bit(&prefix_matched, next_bit as u32);
        }

        prefix_matched as i32
    }


    fn clamp_count(l: u32, r: u32, n: u32) -> u32 {
        if r == 0 || l > n {
            return 0;
        }
        let lo = l.max(1);
        let hi = r.min(n);
        if hi >= lo { hi - lo + 1 } else { 0 }
    }

    fn append_bit(n: &u32, bit: u32) -> u32 {
        (n << 1) | (bit & 1)
    }

    fn get_bit(n: &i32, i: &u32) -> u32 {
        ((*n as u32) >> i) & 1
    }

    fn match_prefix(n: &i32, prefix: &u32, bits_p: u32, bits_n: u32) -> bool {
        if bits_p == 0 {
            return true;
        }
        let n_val = *n as u32;
        let n_prefix = n_val >> (bits_n - bits_p);
        n_prefix == *prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_even() {
        assert_eq!(FirstSolution::find_duplicate(vec![1, 2, 3, 2, 4]), 2);
    }

    #[test]
    fn test_even() {
        assert_eq!(FirstSolution::find_duplicate(vec![1, 2, 3, 2]), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(FirstSolution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }

    #[test]
    fn test_1() {
        assert_eq!(FirstSolution::find_duplicate(vec![1, 3, 4, 2, 1]), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(FirstSolution::find_duplicate(vec![1, 3, 4, 2, 4]), 4);
    }
}
