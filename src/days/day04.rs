use super::advent::problem::Problem;
use crate::{part, run};

use std::collections::HashMap;
use std::ops::Range;

use colored::*;
use itertools;

fn is_valid(n: u64, cmp: fn(usize) -> bool) -> bool {
    let ascending = itertools::equal(
        n.to_string().chars(),
        itertools::sorted(n.to_string().chars()),
    );

    let counts = n.to_string().chars().fold(HashMap::new(), |mut a, c| {
        *a.entry(c).or_insert(0) += 1;
        a
    });

    ascending && counts.values().any(|&v| cmp(v))
}

fn part1_cmp(c: usize) -> bool {
    c >= 2
}

fn part2_cmp(c: usize) -> bool {
    c == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_part1() {
        assert_eq!(is_valid(111_111, part1_cmp), true);
        assert_eq!(is_valid(223_450, part1_cmp), false);
        assert_eq!(is_valid(123_789, part1_cmp), false);
    }

    #[test]
    fn test_is_valid_part2() {
        assert_eq!(is_valid(112_233, part2_cmp), true);
        assert_eq!(is_valid(123_444, part2_cmp), false);
        assert_eq!(is_valid(111_122, part2_cmp), true);
    }
}

pub fn part1() {
    part!(1);

    let problem = Problem::<_, usize>::new(|range: Range<u64>| {
        let mut count = 0;
        for i in range {
            if is_valid(i, part1_cmp) {
                count += 1;
            }
        }
        count
    });

    // tests
    run!();

    run!(
        109_165..576_723 => problem
    );
}

pub fn part2() {
    part!(2);

    let problem = Problem::<_, usize>::new(|range: Range<u64>| {
        let mut count = 0;
        for i in range {
            if is_valid(i, part2_cmp) {
                count += 1;
            }
        }
        count
    });

    // tests
    run!();

    run!(
        109_165..576_723 => problem
    );
}
