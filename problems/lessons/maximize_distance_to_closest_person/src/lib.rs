struct BsSolution;

impl BsSolution {
    // T(n) = O(n * log(n)) with n = n_seats because we perform a binary search on a range [0,..., n_seats - 1]
    // and every check has a complexity O(n) because we check all people sitting that are potentially n_seats
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let index_people_sitting: Vec<usize> = seats
            .iter()
            .enumerate()
            .filter(|&(_, v)| *v != 0)
            .map(|(index, _)| index)
            .collect();
        let first_person_index = index_people_sitting[0];
        let last_person_index = index_people_sitting.last().unwrap();
        let opt_distance_b_person: Vec<usize> = index_people_sitting
            .windows(2)
            .map(|v| {
                let middle = v[0] + (v[1] - v[0]) / 2;
                middle - v[0]
            })
            .collect();
        // binary search on a range [0, num_seats) of possible distance and check if they
        // satisfy the predicate
        let mut low = 0;
        let mut high = seats.len() - 1;
        let mut dist = 0;
        while low <= high {
            let middle = low + (high - low) / 2;
            if Self::check_if_distance_is_possible(
                middle,
                &first_person_index,
                &last_person_index,
                &opt_distance_b_person,
                seats.len()
            ) {
                low = middle + 1;
                dist = middle;
            } else {
                high = middle - 1;
            }
        }
        dist as i32
    }

    fn check_if_distance_is_possible(
        d: usize,
        first_person_index: &usize,
        last_person_index: &usize,
        opt_distance_b_person: &Vec<usize>,
        n_seats: usize,
    ) -> bool {
        *last_person_index + d <= n_seats - 1
            || *first_person_index >= d
            || opt_distance_b_person.iter().any(|&opt_d| opt_d >= d)
    }
}

struct LinearSolution;

impl LinearSolution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let index_people_sitting: Vec<usize> = seats
            .iter()
            .enumerate()
            .filter(|&(_, v)| *v != 0)
            .map(|(index, _)| index)
            .collect();
        let first_person_index = index_people_sitting[0];
        let last_person_index = index_people_sitting.last().unwrap();
        let opt_distance_b_person: Vec<usize> = index_people_sitting
            .windows(2)
            .map(|v| {
                let middle = v[0] + (v[1] - v[0]) / 2;
                middle - v[0]
            })
            .collect();
        let max_between_people = *(opt_distance_b_person.iter().max().unwrap_or(&0)) as i32;

        max_between_people
            .max(first_person_index as i32)
            .max((seats.len() - 1 - last_person_index) as i32)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_d2() {
        assert_eq!(
            2,
            BsSolution::max_dist_to_closest(Vec::from([1, 0, 0, 0, 1, 0, 1]))
        );
        assert_eq!(
            2,
            LinearSolution::max_dist_to_closest(Vec::from([1, 0, 0, 0, 1, 0, 1]))
        );
    }

    #[test]
    fn check_one_people() {
        assert_eq!(3, BsSolution::max_dist_to_closest(Vec::from([1, 0, 0, 0])));
        assert_eq!(
            3,
            LinearSolution::max_dist_to_closest(Vec::from([1, 0, 0, 0]))
        );
    }
}
