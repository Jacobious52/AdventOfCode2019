use super::advent::{problem::Problem};
use crate::{part, run};

use colored::*;


pub fn part1() {
    part!(1);

    let problem = Problem::<_, usize>::new(|args: (Vec<usize>, usize, usize)| {
        
        let digits = args.0;
        let width = args.1;
        let height = args.2;

        let mut curr_min = 1_000_000;
        let mut result = 0;
        let mut img = vec![];
        let mut i = 0;
        while i < digits.len() {
            let mut layer = vec![vec!["".to_owned(); width]; height];
            let mut counts = vec![0, 0, 0];
            for h in 0..height {
                for w in 0..width {
                    
                    let digit = digits[i];
                    match digit {
                        0 | 1 | 2 => counts[digit] += 1,
                        _ => {}
                    };
                   
                    layer[h][w] = digit.to_string();

                    i += 1;
                }
            }
            if counts[0] < curr_min {
                curr_min = counts[0];
                result = counts[1] * counts[2];
            }

            img.push(layer);
        }

        let mut pic = vec![vec![".".to_owned(); width]; height];
        for layer in img.iter().rev() {
            for h in 0..height {
                for w in 0..width {
                    if pic[h][w] == "." || pic[h][w] == "2" {
                        pic[h][w] = layer[h][w].clone();
                    }
                }
            }
        }

        for row in pic {
            println!("{}", row.join(""));
        }

        result
    });

    // tests
    run!(
        ("123456789012".chars().map(|c| c.to_string().parse().unwrap()).collect(), 3, 2) => 1 => problem
    );

    let input = include_str!("../../inputs/day08.txt");

    run!(
        (input.chars().map(|c| c.to_string().parse().unwrap()).collect(), 25, 6) => problem
    );
}

pub fn part2() {
    part!(2);

}
