use super::advent::{
    problem::Problem,
    util
};
use crate::{run, part};

use colored::*;

pub fn part1() {
    part!(1);
    let input = include_str!("../../inputs/day02.txt");
    
    let problem = Problem::<_, i64>::new(|mut nums: Vec<i64>| {
        let mut pos = 0;
        while nums[pos] != 99 && pos < nums.len() {
            let op = nums[pos];
            let a = nums[pos+1] as usize;
            let b = nums[pos+2] as usize;
            let dest = nums[pos+3] as usize;

            nums[dest] = match op {
                1 => nums[a] + nums[b],
                2 => nums[a] * nums[b],
                _ => panic!("invalid op code")
            };

            pos += 4;
        }
        nums[0]
    });

    // tests
    run!(
        vec![1,9,10,3,2,3,11,0,99,30,40,50] => 3500 => problem,
        vec![1,0,0,0,99] => 2 => problem,
        vec![2,3,0,3,99] => 2 => problem,
        vec![2,4,4,5,99,0] => 2 => problem,
        vec![1,1,1,4,99,5,6,0,99] => 30 => problem
    );

    let mut nums = util::numbers_commas(input);
    nums[1] = 12;
    nums[2] = 2;

    // answer
    run!(
        nums => problem
    );
}

pub fn part2() {
    part!(2);
    let input = include_str!("../../inputs/day02.txt");
    
    let problem = Problem::<_, i64>::new(|mut nums: Vec<i64>| {
        let mut pos = 0;
        while nums[pos] != 99 && pos < nums.len() {
            let op = nums[pos];
            let a = nums[pos+1] as usize;
            let b = nums[pos+2] as usize;
            let dest = nums[pos+3] as usize;

            nums[dest] = match op {
                1 => nums[a] + nums[b],
                2 => nums[a] * nums[b],
                _ => panic!("invalid op code")
            };

            pos += 4;
        }
        nums[0]
    });

    // tests
    run!(

    );

    let mut nums = util::numbers_commas(input);
    let mnums = &mut nums;
    for i in 0..100 {
        for j in 0..100 {
            mnums[1] = i;
            mnums[2] = j;

            if problem.run(mnums.to_vec()) == 19_690_720 {
                println!("{} {} = {}", i, j, 100 * i + j);
                return
            }
        }
    }
}
