extern crate advent_of_code_2019;

use advent_of_code_2019::Problem;

fn main() {
    
    let input = include_str!("../../inputs/playground.txt");
    
    let problem = Problem::new(|s: &str| {
        s.lines().count()
    });

    println!("{}", problem.test(input, 9));
}
