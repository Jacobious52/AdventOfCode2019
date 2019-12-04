use super::advent::{problem::Problem, util};
use crate::{part, run};

use colored::*;

fn fuel(m: i64) -> i64 {
    (m / 3) - 2
}

pub fn part1() {
    part!(1);
    let input = include_str!("../../inputs/day01.txt");

    let problem = Problem::<_, i64>::new(|nums: Vec<i64>| nums.into_iter().map(fuel).sum());

    // tests
    run!(
        vec![12] => 2 => problem,
        vec![14] => 2 => problem,
        vec![1969] => 654 => problem,
        vec![100_756] => 33583 => problem
    );

    // answer
    run!(
        util::numbers(input) => problem
    );
}

pub fn part2() {
    part!(2);
    let input = include_str!("../../inputs/day01.txt");

    let problem = Problem::<_, i64>::new(|nums: Vec<i64>| {
        nums.iter()
            .map(|&m| {
                let mut t = 0;
                let mut f = fuel(m);
                while f > 0 {
                    t += f;
                    f = fuel(f);
                }
                t
            })
            .sum()
    });

    // tests
    run!(
        vec![14] => 2 => problem,
        vec![1969] => 966 => problem,
        vec![100_756] => 50346 => problem
    );

    // answer
    run!(
        util::numbers(input) => problem
    );
}
