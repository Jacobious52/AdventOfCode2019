use super::advent::{
    problem::Problem,
    intcode::Interpreter
};
use crate::{run, part};

use colored::*;

use std::str::FromStr;

pub fn part1() {
    part!(1);
    let input = include_str!("../../inputs/day02.txt");
    
    let problem = Problem::<_, usize>::new(|interpreter: Interpreter| {
        interpreter.eval().first()
    });

    // tests
    run!(
        Interpreter::new(vec![1,9,10,3,2,3,11,0,99,30,40,50]) => 3500 => problem,
        Interpreter::new(vec![1,0,0,0,99]) => 2 => problem,
        Interpreter::new(vec![2,3,0,3,99]) => 2 => problem,
        Interpreter::new(vec![2,4,4,5,99,0]) => 2 => problem,
        Interpreter::new(vec![1,1,1,4,99,5,6,0,99]) => 30 => problem
    );

    let mut interpreter = Interpreter::from_str(input).expect("failed to load program");
    interpreter.set(1, 12);
    interpreter.set(2, 2);

    // answer 6568671
    run!(
        interpreter => problem
    );
}

pub fn part2() {
    part!(2);
    let input = include_str!("../../inputs/day02.txt");
    
    let problem = Problem::<_, usize>::new(|interpreter: Interpreter| {
        for i in 0..100 {
            for j in 0..100 {
                let mut interpreter = interpreter.clone();
                interpreter.set(1, i);
                interpreter.set(2, j);
                if interpreter.eval().first() == 19_690_720 {
                    return 100 * i + j
                }
            }
        }
        panic!("no solution found");
    });

    let interpreter = Interpreter::from_str(input).expect("failed to load program");

    // answer: 39 51 = 3951
    run!(
        interpreter => problem
    );
}
