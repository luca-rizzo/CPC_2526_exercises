/* ---------  Problem #1: Holiday Planning  ---------------- */
pub mod holiday_planning {
    use std::cmp::max;
    use std::error::Error;
    use std::mem;

    // in cities_attrs[i][j] we find the attractions that you can visit in city i
    // in day j. ex cities_attrs[0][3] = 5 -> you could see 5 attraction in day 4 in city 1
    fn max_visitable_attractions(cities_attrs: &[Vec<u32>]) -> u32 {
        let n = cities_attrs.len();
        let d = cities_attrs.first().map(|c| c.len()).unwrap_or(0);
        let mut prev: Vec<u32> = vec![0u32; d + 1];
        let mut curr: Vec<u32> = vec![0u32; d + 1];
        for i in 1..(n + 1) {
            let city = &cities_attrs[i - 1];
            for av_days in 1..(d + 1) {
                // let's initialize it with the choice of not consider this city in our trip
                curr[av_days] = prev[av_days];
                let mut cum_attrs = 0;
                // For every feasible choice 'day' (1 ≤ day ≤ av_days) we consider the
                // option of staying exactly 'day' days in the current city.
                // This grants the cumulative attractions of its first 'day' entries.
                // Because those 'day' days consume part of the total 'av_days',
                // the remaining 'av_days - day' must be filled using the best solution
                // computed for the previous cities (stored in 'prev').
                for day in 1..(av_days + 1) {
                    cum_attrs += city[day - 1];
                    curr[av_days] = max(prev[av_days - day] + cum_attrs, curr[av_days])
                }
            }
            mem::swap(&mut curr, &mut prev);
        }
        prev[d]
    }

    pub fn solve(input: &str) -> Result<u32, Box<dyn Error>> {
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().ok_or("missing n")?.parse()?;
        let d: usize = iter.next().ok_or("missing D")?.parse()?;
        let arr: Vec<Vec<u32>> = (0..n)
            .map(|_| {
                (0..d)
                    .map(|_| {
                        let s = iter.next().ok_or("missing value")?;
                        let v = s.parse::<u32>()?;
                        Ok(v)
                    })
                    .collect::<Result<Vec<u32>, Box<dyn Error>>>()
            })
            .collect::<Result<_, _>>()?;
        let output = max_visitable_attractions(&arr);
        Ok(output)
    }

    #[cfg(test)]
    mod tests {
        use crate::holiday_planning::solve;

        fn run_case(i: usize) -> Result<(), Box<dyn std::error::Error>> {
            let input =
                std::fs::read_to_string(format!("./test_sets/holiday_planning/input{i}.txt"))?;
            let expected =
                std::fs::read_to_string(format!("./test_sets/holiday_planning/output{i}.txt"))?
                    .split_whitespace()
                    .next()
                    .expect("output not present")
                    .parse()?;
            let actual = solve(&input)?;
            assert_eq!(actual, expected, "Mismatch on case {i}");
            Ok(())
        }

        #[test]
        fn case0() -> Result<(), Box<dyn std::error::Error>> {
            run_case(0)
        }
        #[test]
        fn case1() -> Result<(), Box<dyn std::error::Error>> {
            run_case(1)
        }
        #[test]
        fn case2() -> Result<(), Box<dyn std::error::Error>> {
            run_case(2)
        }
        #[test]
        fn case3() -> Result<(), Box<dyn std::error::Error>> {
            run_case(3)
        }
        #[test]
        fn case4() -> Result<(), Box<dyn std::error::Error>> {
            run_case(4)
        }
    }
}

/* ---------  Problem #2: Design a course  ----------------- */
pub mod course_design {
    use std::error::Error;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Topic {
        beauty: u32,
        difficulty: u32,
    }

    impl Topic {
        pub fn new(b: u32, d: u32) -> Self {
            Self {
                beauty: b,
                difficulty: d,
            }
        }
    }

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

    pub fn design_course(topics: &[Topic]) -> u32 {
        // I create a copy in order to not mutate the input array
        let mut local_topic: Vec<Topic> = topics.to_vec();
        local_topic.sort_unstable_by(|a, b| {
            a.difficulty
                .cmp(&b.difficulty)
                // When difficulties are equal, we sort by beauty in descending order.
                // This ensures that any topic with the same difficulty appears later
                // with a strictly smaller beauty, preventing the LIS from extending
                // through two topics of equal difficulty. In practice, this makes it
                // impossible for the LIS to pick two topics with equal difficulty.
                .then(b.beauty.cmp(&a.beauty))
        });
        // we extract only the beauty of each topic and then search the longest increasing subsequence
        // in beauties (since we have already sorted by difficulty the overall result will be the longest increasing subsequence in both
        // beauty and difficulty)
        let beauties: Vec<u32> = local_topic.iter().map(|t| t.beauty).collect();
        length_of_lis(&beauties)
    }

    pub fn solve(input: &str) -> Result<u32, Box<dyn Error>> {
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().ok_or("missing n")?.parse()?;
        let arr: Vec<Topic> = (0..n)
            .map(|_| {
                let b = iter.next().ok_or("missing value b for topic")?.parse()?;
                let d = iter.next().ok_or("missing value d for topic")?.parse()?;
                Ok::<Topic, Box<dyn Error>>(Topic::new(b, d))
            })
            .collect::<Result<_, _>>()?;
        let output = design_course(&arr);
        Ok(output)
    }

    #[cfg(test)]
    mod tests {
        use crate::course_design::solve;

        fn run_case(i: usize) -> Result<(), Box<dyn std::error::Error>> {
            let input = std::fs::read_to_string(format!("./test_sets/course_design/input{i}.txt"))?;
            let expected =
                std::fs::read_to_string(format!("./test_sets/course_design/output{i}.txt"))?
                    .split_whitespace()
                    .next()
                    .expect("output not present")
                    .parse()?;
            let actual = solve(&input)?;
            assert_eq!(
                actual, expected,
                "Mismatch on case {i} -> actual: {actual} expected {expected}"
            );
            Ok(())
        }

        #[test]
        fn case0() -> Result<(), Box<dyn std::error::Error>> {
            run_case(0)
        }
        #[test]
        fn case1() -> Result<(), Box<dyn std::error::Error>> {
            run_case(1)
        }
        #[test]
        fn case2() -> Result<(), Box<dyn std::error::Error>> {
            run_case(2)
        }
        #[test]
        fn case3() -> Result<(), Box<dyn std::error::Error>> {
            run_case(3)
        }
        #[test]
        fn case4() -> Result<(), Box<dyn std::error::Error>> {
            run_case(4)
        }

        #[test]
        fn case5() -> Result<(), Box<dyn std::error::Error>> {
            run_case(5)
        }
        #[test]
        fn case6() -> Result<(), Box<dyn std::error::Error>> {
            run_case(6)
        }
        #[test]
        fn case7() -> Result<(), Box<dyn std::error::Error>> {
            run_case(7)
        }
        #[test]
        fn case8() -> Result<(), Box<dyn std::error::Error>> {
            run_case(8)
        }
        #[test]
        fn case9() -> Result<(), Box<dyn std::error::Error>> {
            run_case(9)
        }
        #[test]
        fn case10() -> Result<(), Box<dyn std::error::Error>> {
            run_case(10)
        }
    }
}
