pub mod holiday_planning {
    use std::cmp::max;
    use std::error::Error;
    use std::mem;

    // in cities_attrs[i][j] we find the attractions that you can visit in city i
    // in day j. ex cities_attrs[0][3] = 4 -> you can see 4 attraction in day 4 in city 0
    fn max_visitable_attractions(cities_attrs: Vec<Vec<u32>>) -> u32 {
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
        let output = max_visitable_attractions(arr);
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
