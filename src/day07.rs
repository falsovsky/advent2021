#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::cmp::Ordering;


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
    input.sort_unstable();
    input
}

fn solve_part1(input: &[u64]) -> u64 {
    let mut fuel: u64 = 0;
    let mid = input.len() / 2;
    let magic = input[mid];
    for crab in input {
        match crab.cmp(&magic) {
            Ordering::Greater => { fuel += *crab - magic },
            Ordering::Less => { fuel += magic - *crab }
            _ =>  continue
        }
    }
    fuel
}

fn solve_part2(input: &[u64]) -> u64 {
    let mut fuel: u64 = u64::MAX;
    let mut sum: u64 = 0;
    for x in input {
        sum += x;
    }
    let avg = sum as f32 / input.len() as f32;
    let magics: Vec<u64> = vec![avg.floor() as u64, avg.ceil() as u64];
    for magic in magics {
        let mut thisfuel: u64 = 0;
        for crab in input {
            let start: u64;
            let end: u64;
            match crab.cmp(&magic) {
                Ordering::Greater => {
                    start = magic;
                    end = *crab;
                },
                Ordering::Less => {
                    start = *crab;
                    end = magic;
                }
                _ =>  continue
            }
            /*
            if *crab > magic {
                start = magic;
                end = *crab;
            } else if magic > *crab {
                start = *crab;
                end =  magic;
            }
            */
            let mut add = 1;
            for _ in start..end {
                thisfuel += add;
                add += 1;
            }
        }
        if thisfuel < fuel {
            fuel = thisfuel;
        }
    }
    fuel
}

fn solve(lines: &[u64], parts: u8) -> (u64, u64) {
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
        let mut input = CODE.to_vec();
        input.sort();
        let (pt1, _) = solve(&input, PART1);
        assert_eq!(pt1, 37);
    }

    #[test]
    fn part2() {
        let mut input = CODE.to_vec();
        input.sort();
        let (_, pt2) = solve(&input, PART2);
        assert_eq!(pt2, 168);

    }

    #[bench]
    fn bench_day07(b: &mut test::Bencher) {
        b.iter(|| main());
    }
}
