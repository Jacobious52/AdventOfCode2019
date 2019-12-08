use super::advent::{intcode::Interpreter, problem::Problem};
use crate::{part, run};

use colored::*;

use itertools::Itertools;
use std::str::FromStr;
use std::collections::VecDeque;

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
                let mut input = VecDeque::from(vec![phase as isize, output]);
                let mut out_mem = Vec::new();
                //dbg!(&input);
                amp.eval(&mut input, &mut out_mem, false);
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

    let problem = Problem::<_, isize>::new(|args: (Interpreter, Vec<isize>)| {
        let interpreter = args.0;
        let phases = args.1;
        let amplifiers: Vec<_> = phases.iter().map(|_| interpreter.clone()).collect();

        let mut final_outputs = Vec::new();
        for order in phases.iter().permutations(phases.len()) {
            let mut output = Some(0);
            let mut input = VecDeque::new();
            let mut stopped = false;
            let mut amps = amplifiers.to_vec();
            let mut give_phases = true;
            while !stopped {
                for (&&phase, amp) in order.iter().zip(amps.iter_mut()) {
                    let mut out_mem = Vec::new();
                    if give_phases {
                        input.push_back(phase);
                    }
                    if let Some(output) = output {
                        input.push_back(output);
                    }
                    //dbg!(&input);
                    *amp = amp.clone().eval(&mut input, &mut out_mem, false);
                    output = out_mem.last().copied();
                    //dbg!(&output);

                    stopped = amp.did_halt();
                }
                give_phases = false;
            }
            final_outputs.push(output);
        }

        final_outputs.into_iter().max().expect("no minimum found").expect("final output is empty")
    });

    // tests
    run!(
        (Interpreter::new(vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]), (5..=9).collect()) => 139_629_729 => problem,
        (Interpreter::new(vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10]), (5..=9).collect()) => 18216 => problem
    );

    let input = include_str!("../../inputs/day07.txt");

    run!(
        (Interpreter::from_str(input).expect("failed to parse input"), (5..=9).collect()) => problem
    );
}
