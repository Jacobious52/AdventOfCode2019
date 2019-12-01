extern crate advent_of_code_2019;

use advent_of_code_2019::Problem;
use advent_of_code_2019::util;
use advent_of_code_2019::{run, part};

use colored::*;

fn fuel(m: i64) -> i64 {
    (m / 3) - 2
}

fn part1() {
    part!(1);
    let input = include_str!("../../inputs/day01part1.txt");
    
    let problem = Problem::<_, i64>::new(|nums: Vec<i64>| {
        nums.iter().map(|n| fuel(*n)).sum()
    });

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

fn part2() {
    part!(2);
    let input = include_str!("../../inputs/day01part2.txt");
    
    let problem = Problem::<_, i64>::new(|nums: Vec<i64>| {
        let totals: Vec<i64> = nums.iter().map(|m| {
            let mut t = 0;
            let mut f = fuel(*m);
            while f > 0 {
                t += f;
                f = fuel(f);
            }
            t
        }).collect();
        
        totals.iter().sum()
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

fn main() {
    part1();
    part2();
}
