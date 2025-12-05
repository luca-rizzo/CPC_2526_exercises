pub mod holiday_planning {
    // in cities_attrs[i][j] we find the attractions that you can visit in city i
    // in day j. ex cities_attrs[0][3] = 4 -> you can see 4 attraction in day 4 in city 0

    use std::cmp::max;
    use std::error::Error;

    struct Item {
        w: u32,
        v: u32,
    }

    impl Item {
        fn new(v: u32, w: u32) -> Self {
            Self { w, v }
        }
    }

    fn max_visitable_attractions(cities_attrs: Vec<Vec<u32>>) -> u32 {
        let n = cities_attrs.len();
        let d = cities_attrs.get(0).map(|c| c.len()).unwrap_or(0);
        let mut items: Vec<Item> = Vec::with_capacity(n * d);
        for city in cities_attrs {
            let mut sum = 0;
            for (idx, num_attr) in city.iter().enumerate() {
                sum += num_attr;
                items.push(Item::new(sum, (idx + 1) as u32));
            }
        }
        let mut t: Vec<Vec<u32>> = vec![vec![0; d + 1]; n * d + 1];
        for i in 1..(n * d + 1) {
            let c_item = &items[i - 1];
            for j in 1..(d + 1) {
                if j < c_item.w as usize {
                    t[i][j] = t[i - 1][j];
                    continue;
                }
                t[i][j] = max(
                    t[i - 1][j],
                    t[i - c_item.w as usize][j - c_item.w as usize] + c_item.v,
                );
            }
        }
        t[n * d][d]
    }

    pub fn solve(input: &str) -> Result<u32, Box<dyn Error>> {
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().ok_or("missing n")?.parse()?;
        let D: usize = iter.next().ok_or("missing D")?.parse()?;
        let arr: Vec<Vec<u32>> = (0..n)
            .map(|_| {
                (0..D)
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
        use crate::holiday_planning::{max_visitable_attractions, solve};

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
