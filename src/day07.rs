#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const PART1: u8 = 0b01;
const PART2: u8 = 0b10;

fn read_input() -> Vec<u64> {
    let filename = "input/day07.txt";
    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let buffer = BufReader::new(&fp);
    let mut input: Vec<u64> = Vec::new();
    for line in buffer.lines() {
        let line_str = match line {
            Ok(value) => value,
            Err(error) => panic!("Could not read anything - {}", error),
        };
        for value in line_str.split(',') {
            input.push(value.parse::<u64>().expect("Could not convert to u64"))
        }
    }
    input
}

fn solve_part1(input: &Vec<u64>) -> u64 {
    let mut fuel: u64 = u64::MAX;
    for magic in input {
        let mut thisfuel: u64 = 0;
        for crab in input {
            if *crab > *magic {
                thisfuel += *crab - *magic;
            } else if *magic > *crab {
                thisfuel += *magic - *crab;
            }
        }
        if thisfuel < fuel && thisfuel != 0 {
            fuel = thisfuel;
        }
    }
    fuel
}

fn solve_part2(input: &Vec<u64>) -> u64 {
    let mut fuel: u64 = u64::MAX;
    for magic in *input.iter().min().unwrap()..*input.iter().max().unwrap() as u64 {
        let mut thisfuel: u64 = 0;
        for crab in input {
            if *crab > magic {
                let mut add = 1;
                for _ in magic..*crab {
                    thisfuel += add;
                    add += 1;
                }
            } else if magic > *crab {
                let mut add = 1;
                for _ in *crab..magic {
                    thisfuel += add;
                    add += 1;
                }
            }
        }
        if thisfuel < fuel && thisfuel != 0 {
            fuel = thisfuel;
        }
    }
    fuel
}

fn solve(lines: &Vec<u64>, parts: u8) -> (u64, u64) {
    let runpt1: bool = parts & PART1 != 0;
    let runpt2: bool = parts & PART2 != 0;
    let mut pt1: u64 = 0;
    let mut pt2: u64 = 0;
    if runpt1 {
        pt1 = solve_part1(lines);
    }
    if runpt2 {
        pt2 = solve_part2(lines);
    }
    (pt1, pt2)
}

fn main() {
    let lines = read_input();
    let (pt1, pt2) = solve(&lines, PART1 | PART2);
    println!("Part1: {:?}", pt1);
    println!("Part2: {:?}", pt2);
}

#[cfg(test)]
mod day07 {
    use crate::*;

    const CODE: [u64; 10] =  [ 16,1,2,0,4,2,7,1,2,14 ];

    #[test]
    fn part1() {
        let input = CODE.to_vec();
        let (pt1, _) = solve(&input, PART1);
        assert_eq!(pt1, 37);
    }

    #[test]
    fn part2() {
        let input = CODE.to_vec();
        let (_, pt2) = solve(&input, PART2);
        assert_eq!(pt2, 168);

    }

    #[bench]
    fn bench_day07(b: &mut test::Bencher) {
        b.iter(|| main());
    }
}
