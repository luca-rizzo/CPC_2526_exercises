fn min_steps(b: &[i32]) -> u32 {
    if b.is_empty() {
        return 0;
    }
    let mut min_steps = b[0];
    // The idea is to scan the array from left to right and
    // at each position i, we apply enough +1 or -1 operations
    // on the suffix to make a[i] equal to b[i]. At the beginning
    // we transform a[0] in b[0] with |b[0]| increments or decrements
    //
    // Since each operation affects the entire suffix [i..n],
    // once we adjust a[i] to b[i], the whole suffix becomes
    // flat and equal to b[i]. Therefore, when we reach position i,
    // we know that a[i] = b[i-1], so transforming a[i] into b[i]
    // requires exactly |b[i] - b[i-1]| operations
    //
    // Hence, the total minimum number of operations is:
    // |b[0]| + sum_{i = 2..n} |b[i] - b[i-1]|
    for i in 1..b.len() {
        min_steps += (b[i] - b[i-1]).abs();
    }
    min_steps as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(min_steps(&[1, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(min_steps(&[1, 2, 2, 1]), 3);
    }
}