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
        interpreter.eval(input, &mut output, false);
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

    let problem = Problem::<_, isize>::new(|args: (Interpreter, Vec<isize>)| {
        let input = args.1;
        let interpreter = args.0;
        let mut output = Vec::new();
        
        interpreter.eval(input, &mut output, false);
    
        *output.last().expect("no diagnostic code")
    });

    // tests
    run!(
        (Interpreter::new(vec![3,9,8,9,10,9,4,9,99,-1,8]), vec![6]) => 0 => problem,
        (Interpreter::new(vec![3,9,8,9,10,9,4,9,99,-1,8]), vec![8]) => 1 => problem,
        (Interpreter::new(vec![3,3,1107,-1,8,3,4,3,99]), vec![11]) => 0 => problem,
        (Interpreter::new(vec![3,3,1107,-1,8,3,4,3,99]), vec![3]) => 1 => problem,
        (Interpreter::new(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]), vec![0]) => 0 => problem,
        (Interpreter::new(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]), vec![3]) => 1 => problem,
        (Interpreter::new(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]), vec![3]) => 999 => problem,
        (Interpreter::new(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]), vec![8]) => 1000 => problem,
        (Interpreter::new(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]), vec![434]) => 1001 => problem
    );

    let input = include_str!("../../inputs/day05.txt");

    run!(
        (Interpreter::from_str(input).expect("failed to parse input"), vec![5]) => problem
    );
}
