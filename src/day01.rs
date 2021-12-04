#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const PART1: u8 = 0b01;
const PART2: u8 = 0b10;

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

fn solve_part1(program: &[u16]) -> u16 {
    let mut pt1: u16 = 0;
    let mut last: u16 = 0;
    for current in program {
        if last > 0 && last < *current {
            pt1 += 1;
        }
        last = *current;
    }
    pt1
}

fn solve_part2(program: &[u16]) -> u16 {
    let mut pt2: u16 = 0;
    let mut last: u16 = 0;
    let mut current: u16;
    let mut pc: usize = 0;
    while pc < program.len() - 2 {
        current = program[pc] + program[pc + 1] + program[pc + 2];
        if last > 0 && last < current {
            pt2 += 1;
        }
        last = current;
        pc += 1;
    }
    pt2
}

fn solve(program: &[u16], parts: u8) -> (u16, u16) {
    let runpt1: bool = parts & PART1 != 0;
    let runpt2: bool = parts & PART2 != 0;
    let mut pt1: u16 = 0;
    let mut pt2: u16 = 0;
    if runpt1 {
        pt1 = solve_part1(program);
    }
    if runpt2 {
        pt2 = solve_part2(program);
    }
    (pt1, pt2)
}

fn main() {
    let code = read_input();
    let (pt1, pt2) = solve(&code, PART1 | PART2);
    println!("Part1: {:?}", pt1);
    println!("Part2: {:?}", pt2);
}

#[cfg(test)]
mod day01 {
    use crate::*;

    const CODE: [u16; 10]  = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn part1() {
        let (pt1, _) = solve(&CODE, PART1);
        assert_eq!(pt1, 7);
    }

    #[test]
    fn part2() {
        let (_, pt2) = solve(&CODE, PART2);
        assert_eq!(pt2, 5);
    }

    #[bench]
    fn bench_day01(b: &mut test::Bencher) {
        b.iter(|| main());
    }
}
