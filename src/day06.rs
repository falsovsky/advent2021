#![feature(test)]

extern crate test;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const PART1: u8 = 0b01;
const PART2: u8 = 0b10;

fn read_input() -> Vec<u8> {
    let filename = "input/day06.txt";
    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let buffer = BufReader::new(&fp);
    let mut input: Vec<u8> = Vec::new();
    for line in buffer.lines() {
        let line_str = match line {
            Ok(value) => value,
            Err(error) => panic!("Could not read anything - {}", error),
        };
        for value in line_str.split(',') {
            input.push(value.parse::<u8>().expect("Could not convert to u8"))
        }
    }
    input
}

fn solver(input: Vec<u8>, size: u16) -> u64 {
    let mut values: HashMap<u8, u64> = HashMap::new();
    // set all numbers at 0
    for x in 0..=8 {
        values.insert(x, 0);
    }
    // read input
    for value in input {
        let mut number: u64 = 1;
        if values.contains_key(&value) {
            number = *values.get(&value).unwrap();
            number += 1;
        }
        values.insert(value, number);
    }
    // iterate
    for _ in 0..size {
        let zero = &values.get(&0).unwrap().clone();
        for idx in 0..=7 {
            values.insert(idx, *values.get(&(&idx + 1)).unwrap());
        }
        values.insert(8, *zero);
        values.insert(6, *values.get(&6).unwrap() + zero);
    }
    // get total
    let mut total: u64 = 0;
    for (_, v) in values.iter() {
        total += v;
    }

    total
}

fn solve(lines: &Vec<u8>, parts: u8) -> (u64, u64) {
    let runpt1: bool = parts & PART1 != 0;
    let runpt2: bool = parts & PART2 != 0;
    let mut pt1: u64 = 0;
    let mut pt2: u64 = 0;
    if runpt1 {
        pt1 = solver(lines.to_vec(), 80);
    }
    if runpt2 {
        pt2 = solver(lines.to_vec(), 256);
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
mod day06 {
    use crate::*;

    const CODE: [u8; 5] = [ 3,4,3,1,2 ];

    #[test]
    fn part1() {
        let input = CODE.to_vec();
        let (pt1, _) = solve(&input, PART1);
        assert_eq!(pt1, 5934);
    }

    #[test]
    fn part2() {
        let input = CODE.to_vec();
        let (_, pt2) = solve(&input, PART2);
        assert_eq!(pt2, 26984457539);

    }

    #[bench]
    fn bench_day06(b: &mut test::Bencher) {
        b.iter(|| main());
    }
}
