use super::advent::{intcode::Interpreter, problem::Problem};
use crate::{part, run};

use std::iter;
use std::str::FromStr;

use colored::*;

pub fn part1() {
    part!(1);

    let problem = Problem::<_, isize>::new(|interpreter: Interpreter| {
        let mut output = Vec::new();
        let input = iter::once(1);
        interpreter.eval(input, &mut output, true);
        dbg!(&output);
        *output.last().expect("no diagnostic code")
    });

    // tests
    run!(
        //Interpreter::new(vec![1002, 4, 3, 4, 33]) => 0 => problem
    );

    let input = include_str!("../../inputs/day05.txt");

    run!(
        Interpreter::from_str(input).expect("failed to parse input") => problem
    );
}

pub fn part2() {
    part!(2);

    let _problem = Problem::<_, usize>::new(|_range: u64| {
        0
    });

    // tests
    run!();

    run!(
        
    );
}
