pub mod holiday_planning {
    use std::cmp::max;
    use std::error::Error;
    use std::mem;

    // in cities_attrs[i][j] we find the attractions that you can visit in city i
    // in day j. ex cities_attrs[0][3] = 4 -> you can see 4 attraction in day 4 in city 0
    fn max_visitable_attractions(cities_attrs: &Vec<Vec<u32>>) -> u32 {
        let n = cities_attrs.len();
        let d = cities_attrs.get(0).map(|c| c.len()).unwrap_or(0);
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

    pub fn length_of_lis(nums: Vec<u32>) -> i32 {
        // We maintain an array `dominant` where dominant[k] holds the smallest possible
        // tail value of an increasing subsequence of length k+1.
        let mut dominant: Vec<Option<u32>> = vec![None; nums.len()];
        for num in nums {
            // for each value we search the number the LIS that we can extend: since dominant
            // is ordered we can use partition_point
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
        dominant
            .iter()
            .rposition(|v| v.is_some())
            .map(|x| x + 1)
            .unwrap_or(0) as i32
    }

    pub fn design_course(topics: &Vec<Topic>) -> u32 {
        let mut local_topic = topics.clone();
        local_topic.sort_unstable_by(|a, b| {
            a.difficulty
                .cmp(&b.difficulty)
                .then(b.beauty.cmp(&a.beauty))
        });
        let beauties: Vec<u32> = local_topic.iter().map(|t| t.beauty).collect();
        length_of_lis(beauties) as u32
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