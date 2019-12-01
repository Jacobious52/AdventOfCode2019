use super::advent::{
    problem::Problem,
    util
};
use crate::{run, part};

use colored::*;

pub fn part1() {
    part!(1);
    let input = include_str!("../../inputs/day02.txt");
    
    let problem = Problem::<_, i64>::new(|_nums: Vec<i64>| {
        0
    });

    // tests
    run!(

    );

    // answer
    run!(
        util::numbers(input) => problem
    );
}

pub fn part2() {
    part!(2);
    let input = include_str!("../../inputs/day02.txt");
    
    let problem = Problem::<_, i64>::new(|_nums: Vec<i64>| {
        0
    });

    // tests
    run!(

    );

    // answer
    run!(
        util::numbers(input) => problem
    );
}
