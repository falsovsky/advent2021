#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const PART1: u8 = 0b01;
const PART2: u8 = 0b10;

type PuzzleInput = Vec<(char, u32)>;

fn read_input() -> PuzzleInput {
    let filename = "input/day02.txt";
    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let buffer = BufReader::new(&fp);
    let mut input: PuzzleInput = Vec::new();
    for line in buffer.lines() {
        let line_str = match line {
            Ok(value) => value,
            Err(error) => panic!("Could not read anything - {}", error),
        };
        let values: Vec<&str> = line_str.split_whitespace().collect();
        let movement: char = values[0].chars().nth(0).expect("Index 0 not found");
        let many: u32 = values[1].parse::<u32>()
            .expect("Could not parse Index 1 to int");
        input.push((movement, many));
    }
    input
}

fn solve_part1(program: &PuzzleInput) -> u32 {
    let mut point: (u32, u32) = (0, 0);
    for instruction in program {
        match instruction.0 {
            'f' => point.1 += instruction.1,
            'd' => point.0 += instruction.1,
            'u' => point.0 -= instruction.1,
            _ => panic!("Invalid instruction: {}", instruction.0),
        }
    }
    point.0 * point.1
}

fn solve_part2(program: &PuzzleInput) -> u32 {
    let mut point: (u32, u32) = (0, 0);
    let mut aim: u32 = 0;
    for instruction in program {
        match instruction.0 {
            'f' => {
                point.1 += instruction.1;
                point.0 += aim * instruction.1
            },
            'd' => aim += instruction.1,
            'u' => aim -= instruction.1,
            _ => panic!("Invalid instruction: {}", instruction.0),
        }
    }
    point.0 * point.1
}

fn solve(program: &PuzzleInput, parts: u8) -> (u32, u32) {
    let runpt1: bool = parts & PART1 != 0;
    let runpt2: bool = parts & PART2 != 0;
    let mut pt1: u32 = 0;
    let mut pt2: u32 = 0;
    if runpt1 {
        pt1 = solve_part1(&program);
    }
    if runpt2 {
        pt2 = solve_part2(&program);
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
mod day02 {
    use crate::*;

    const CODE: [(char, u32);6] = [
        ('f', 5),
        ('d', 5),
        ('f', 8),
        ('u', 3),
        ('d', 8),
        ('f', 2),
    ];

    #[test]
    fn test_part1() {
        let input: PuzzleInput = CODE.to_vec();
        let (pt1, _) = solve(&input, PART1);
        assert_eq!(pt1, 150);
    }

    #[test]
    fn test_part2() {
        let input: PuzzleInput = CODE.to_vec();
        let (_, pt2) = solve(&input, PART2);
        assert_eq!(pt2, 900);
    }

    #[bench]
    fn bench_day02(b: &mut test::Bencher) {
        b.iter(|| main());
    }
}
