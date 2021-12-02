#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Problem {
    part1: bool,
    part2: bool
}

impl Problem {
    pub fn new(value: u8) -> Problem {
        let mut p: Problem = Problem { part1: false, part2: false };
        match value {
            1 => p.part1 = true,
            2 => p.part2 = true,
            3 => {
                p.part1 = true;
                p.part2 = true;
            }
            _ => panic!("Only 1, 2 or 3 - You gave {}", value),
        }
        p
    }
}

fn read_input() -> Vec<u16> {
    let filename = "input/day01.txt";
    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let buffer = BufReader::new(&fp);
    let mut input: Vec<u16> = Vec::new();
    for line in buffer.lines() {
        let line_str = match line {
            Ok(value) => value,
            Err(error) => panic!("Could not read anything - {}", error),
        };
        let value = match line_str.parse::<u16>() {
            Ok(value) => value,
            Err(error) => panic!("Could not convert {} to a integer - {}", line_str, error),
        };
        input.push(value);
    }
    input
}

fn solve(program: &[u16], problem: Problem) -> (u16, u16) {
    let mut pt1: u16 = 0;
    let mut pt2: u16 = 0;
    if problem.part1 {
        let mut last: u16 = 0;
        for current in program {
            if last > 0 && last < *current {
                pt1 += 1;
            }
            last = *current;
        }
    };
    if problem.part2 {
        let mut last: u16 = 0;
        let mut current: u16;
        let mut pc: u16 = 0;
        while pc < (program.len() - 2) as u16 {
            current = program[pc as usize] + program[(pc + 1) as usize]
                + program[(pc + 2) as usize];
            if last > 0 && last < current {
                pt2 += 1;
            }
            last = current;
            pc += 1;
        }
    }
    (pt1, pt2)
}

fn main() {
    let code = read_input();
    let parts = Problem::new(3);
    let (pt1, pt2) = solve(&code, parts);
    println!("Part1: {:?}", pt1);
    println!("Part2: {:?}", pt2);
}

#[cfg(test)]
mod tests {
    use crate::*;

    const CODE: [u16; 10]  = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn part1() {
        let (pt1, _) = solve(&CODE, Problem::new(1));
        assert_eq!(pt1, 7);
    }

    #[test]
    fn part2() {
        let (_, pt2) = solve(&CODE, Problem::new(2));
        assert_eq!(pt2, 5);
    }
}

#[bench]
fn bench_day01(b: &mut test::Bencher) {
    b.iter(|| main());
}
