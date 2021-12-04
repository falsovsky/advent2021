#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const PART1: u8 = 0b01;
const PART2: u8 = 0b10;

fn read_input() -> Vec<u32> {
    let filename = "input/day03.txt";
    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let buffer = BufReader::new(&fp);
    let mut input: Vec<u32> = Vec::new();
    for line in buffer.lines() {
        let line_str = match line {
            Ok(value) => value,
            Err(error) => panic!("Could not read anything - {}", error),
        };
        let intval = isize::from_str_radix(&line_str, 2).expect("Could not convert {} to u32") as u32;
        input.push(intval);
    }
    input
}

fn solve_part1(program: &Vec<u32>, size: usize) -> u32 {
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for bit in (0..size).rev() {
        let mut common: (u16, u16) = (0, 0);
        for item in program {
            if item & (1 << bit) == 0 {
                common.0 += 1;
            } else {
                common.1 += 1;
            }
        }
        if common.1 > common.0 {
            gamma |= 1 << bit;
        } else {
            epsilon |= 1 << bit;
        }
    }
    gamma * epsilon
}

fn solve_part2(program: &Vec<u32>, size: usize) -> u32 {
    let mut p2 = program.to_owned();
    for bit in (0..size).rev() {
        let mut common: (u16, u16) = (0, 0);
        let mut keep: bool = false;
        for item in &p2 {
            if item & (1 << bit) == 0 {
                common.0 += 1;
            } else {
                common.1 += 1;
            }
        }
        if common.1 > common.0 || common.0 == common.1 {
            keep = true;
        }
        if p2.len() > 1 {
            p2.retain(|&i| (i & (1 << bit) != 0) == keep);
        }
    }
    let oxygen: u32 = p2[0];

    let mut p2 = program.to_owned();
    for bit in (0..size).rev() {
        let mut common: (u16, u16) = (0, 0);
        let mut keep: bool = true;
        for item in &p2 {
            if item & (1 << bit) == 0 {
                common.0 += 1;
            } else {
                common.1 += 1;
            }
        }
        if common.0 <= common.1 {
            keep = false;
        }
        if p2.len() > 1 {
            p2.retain(|&i| (i & (1 << bit) != 0) == keep);
        }
    }
    let co2: u32 = p2[0];

    oxygen * co2
}

fn solve(program: &Vec<u32>, size: usize, parts: u8) -> (u32, u32) {
    let runpt1: bool = parts & PART1 != 0;
    let runpt2: bool = parts & PART2 != 0;
    let mut pt1: u32 = 0;
    let mut pt2: u32 = 0;
    if runpt1 {
        pt1 = solve_part1(program, size);
    }
    if runpt2 {
        pt2 = solve_part2(program, size);
    }
    (pt1, pt2)
}

fn main() {
    let code = read_input();
    let (pt1, pt2) = solve(&code, 12, PART1 | PART2);
    println!("Part1: {:?}", pt1);
    println!("Part2: {:?}", pt2);
}

#[cfg(test)]
mod day02 {
    use crate::*;

    const CODE: [u32; 12] = [
        0b00100,
        0b11110,
        0b10110,
        0b10111,
        0b10101,
        0b01111,
        0b00111,
        0b11100,
        0b10000,
        0b11001,
        0b00010,
        0b01010
    ];

    #[test]
    fn test_part1() {
        let input = CODE.to_vec();
        let (pt1, _) = solve(&input, 5, PART1);
        assert_eq!(pt1, 198);
    }

    #[test]
    fn test_part2() {
        let input = CODE.to_vec();
        let (_, pt2) = solve(&input, 5, PART2);
        assert_eq!(pt2, 230);
    }

    #[bench]
    fn bench_day03(b: &mut test::Bencher) {
        b.iter(|| main());
    }
}
