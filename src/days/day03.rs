use super::advent::problem::Problem;
use crate::{part, run};

use colored::*;

use std::collections::{HashMap, HashSet};

fn parse_input(s: &str) -> Vec<Vec<(String, usize)>> {
    s.lines()
        .map(|l| {
            l.split(',')
                .map(|w| (w[..1].to_string(), w[1..].parse().unwrap()))
                .collect()
        })
        .collect()
}

fn draw_wire(wire: &[(String, usize)]) -> HashMap<(i64, i64), usize> {
    let mut map = HashMap::new();
    let (mut x, mut y) = (0i64, 0i64);
    let mut length = 0;
    for turn in wire {
        let dl = turn.1;
        let (dx, dy) = match &*turn.0 {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => panic!("unknown direction"),
        };

        for _ in 0..dl {
            x += dx;
            y += dy;
            length += 1;
            map.entry((x, y)).or_insert(length);
        }
    }
    map
}

pub fn part1() {
    part!(1);
    let input = include_str!("../../inputs/day03.txt");
    let wires = parse_input(input);

    let problem = Problem::<_, usize>::new(|wires: Vec<Vec<(String, usize)>>| {
        let mut wire_points = Vec::new();
        for wire in wires {
            wire_points.push(draw_wire(&wire));
        }

        let mut min = 100_000;
        for i in 0..wire_points.len() {
            for j in i + 1..wire_points.len() {
                let a: HashSet<_> = wire_points[i].keys().collect();
                let b: HashSet<_> = wire_points[j].keys().collect();
                let both = a.intersection(&b);
                let min_point = both
                    .min_by(|x, y| (x.0.abs() + x.1.abs()).cmp(&(y.0.abs() + y.1.abs())))
                    .unwrap_or(&&(0, 0));
                let dist = (min_point.0.abs() + min_point.1.abs()) as usize;
                if dist < min {
                    min = dist;
                }
            }
        }

        min
    });

    // tests
    run!(
        parse_input("R8,U5,L5,D3\nU7,R6,D4,L4") => 6 => problem
    );

    // answer 6568671
    run!(
        wires => problem
    );
}

pub fn part2() {
    part!(2);
    let input = include_str!("../../inputs/day03.txt");
    let wires = parse_input(input);

    let problem = Problem::<_, usize>::new(|wires: Vec<Vec<(String, usize)>>| {
        let mut wire_points = Vec::new();
        for wire in wires {
            wire_points.push(draw_wire(&wire));
        }

        let mut min = 100_000;
        for i in 0..wire_points.len() {
            for j in i + 1..wire_points.len() {
                let a: HashSet<_> = wire_points[i].keys().collect();
                let b: HashSet<_> = wire_points[j].keys().collect();
                let both = a.intersection(&b);
                let dist = both
                    .map(|x| wire_points[i][x] + wire_points[j][x])
                    .min()
                    .unwrap_or(0);
                if dist < min {
                    min = dist;
                }
            }
        }

        min
    });

    // tests
    run!(
        parse_input("R8,U5,L5,D3\nU7,R6,D4,L4") => 30 => problem
    );

    // answer 6568671
    run!(
        wires => problem
    );
}
