use super::advent::{intcode::Interpreter, problem::Problem};
use crate::{part, run};

use colored::*;

use itertools::Itertools;
use std::str::FromStr;

pub fn part1() {
    part!(1);

    let problem = Problem::<_, isize>::new(|args: (Interpreter, usize)| {
        let interpreter = args.0;
        let num_amps = args.1;
        let amplifiers: Vec<_> = (0..=num_amps).map(|_| interpreter.clone()).collect();
        let phases: Vec<_> = (0..=num_amps).collect();

        let mut final_outputs = Vec::new();
        for order in phases.iter().permutations(phases.len()) {
            let mut output = 0;
            for (&&phase, amp) in order.iter().zip(amplifiers.iter().cloned()) {
                let input = vec![phase as isize, output];
                let mut out_mem = Vec::new();
                //dbg!(&input);
                amp.eval(input, &mut out_mem, false);
                //dbg!(&output);
                output = *out_mem.last().expect("no output from program");
            }
            final_outputs.push(output);
        }

        final_outputs.into_iter().max().expect("no minimum found")
    });

    // tests
    run!(
        (Interpreter::new(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]), 4) => 43210 => problem
    );

    let input = include_str!("../../inputs/day07.txt");

    run!(
        (Interpreter::from_str(input).expect("failed to parse input"), 4) => problem
    );
}

pub fn part2() {
    part!(2);
}
