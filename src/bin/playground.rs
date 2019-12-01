extern crate advent_of_code_2019;

use advent_of_code_2019::Problem;
use advent_of_code_2019::util;
use advent_of_code_2019::run;

use colored::*;

fn main() {
    
    let input = include_str!("../../inputs/playground.txt");
    
    let problem = Problem::<_, i64>::new(|nums: Vec<i64>| {
        nums.iter().sum()
    });

    run!(
        vec![1,2,3,4,5] => 15 => problem,
        vec![1,2,3,4,5] => 20 => problem
    );

    run!(
        util::numbers(input) => problem
    );
}
