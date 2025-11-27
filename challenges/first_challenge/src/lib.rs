mod first_exercise {

    fn max_parenthesis_depth(to_check: &str) -> i32 {
        to_check
            .chars()
            .scan(0, |depth, item| {
                *depth = match item {
                    '(' => *depth + 1,
                    ')' => *depth - 1,
                    _ => *depth,
                };
                Some(*depth)
            })
            .max()
            .unwrap_or(0)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn depth_3() {
            let string = "((()))";
            assert_eq!(max_parenthesis_depth(string), 3);
        }

        #[test]
        fn depth_2() {
            let string = "(aas(ddd))ff";
            assert_eq!(max_parenthesis_depth(string), 2);
        }

        #[test]
        fn depth_1() {
            let string = "(fasljfasdkljfhaslkjfhaklfjh)";
            assert_eq!(max_parenthesis_depth(string), 1);
        }
    }
}

mod second_exercise {

    fn missing_number(perm: &[i32]) -> i32 {
        perm.iter()
            .enumerate()
            //we start from perm.len() cause we always have a number less so the last index is not summed up
            //in fold to obtain the sum of all number in the permutation
            .fold(perm.len() as i32, |acc, (i, &x)| acc + (i as i32) - x)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn miss3() {
            let perm = [0, 4, 1, 2];
            assert_eq!(missing_number(&perm), 3);
        }

        #[test]
        fn miss2() {
            let perm = [0, 4, 1, 3];
            assert_eq!(missing_number(&perm), 2);
        }

        #[test]
        fn miss_n() {
            let perm = [0, 2, 1, 3];
            assert_eq!(missing_number(&perm), 4);
        }
    }
}
