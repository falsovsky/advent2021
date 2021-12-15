#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

const PART1: u8 = 0b01;
const PART2: u8 = 0b10;

#[derive(Debug)]
struct Input {
    template: String,
    rules: HashMap<String, String>
}

fn read_input(filename: &str) -> Input {
    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let buffer = BufReader::new(&fp);
    let mut template: String = String::new();
    let mut rules: HashMap<String, String> = HashMap::new();
    for (idx, line) in buffer.lines().enumerate() {
        let line_str = match line {
            Ok(value) => value,
            Err(error) => panic!("Could not read anything - {}", error),
        };
        if idx == 0 {
            template = line_str;
            continue;
        }
        if idx == 1 {
            continue;
        }

        let values = line_str.split(" -> ").collect::<Vec<&str>>();
        rules.insert(values[0].to_string(), values[1].to_string());

    }
    Input { template, rules }
}

fn solve_part1(input: &Input) -> u64 {
    let mut old: String = input.template.clone();
    let mut new: String = String::new();
    for _ in 0..10 {
        let mut idx = 0;
        while idx < old.len() - 1 {
            let pair: String = old[idx..=idx+1].to_string();
            if input.rules.contains_key(&pair) {
                let middle: String = input.rules.get(&pair).unwrap().to_string();
                if idx == 0 {
                    new += &pair[0..1];
                }
                new += &middle;
                new += &pair[1..2];
            } else {
                new += &pair;
            }
            idx += 1;
        }
        old = new;
        new = String::new();
    }

    let total: Vec<char> = old.chars().collect();
    let mut letters = total.clone();
    let mut sorted: Vec<(char, u64)> = Vec::new();
    letters.sort_unstable();
    letters.dedup();
    for letter in letters {
        let count = total.iter().filter(|&n| *n == letter).count() as u64;
        sorted.push((letter, count));
    }
    sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    (sorted[sorted.len() - 1].1 - sorted[0].1) as u64
}

fn solve_part2(input: &Input) -> u64 {
    let mut old: String = input.template.clone();
    let mut new: String = String::new();
    for idx in 0..40 {
        println!("SOFAGEM {}", idx);
        let mut idx = 0;
        while idx < old.len() - 1 {
            let pair: String = old[idx..=idx+1].to_string();
            if input.rules.contains_key(&pair) {
                let middle: String = input.rules.get(&pair).unwrap().to_string();
                if idx == 0 {
                    new += &pair[0..1];
                }
                new += &middle;
                new += &pair[1..2];
            } else {
                new += &pair;
            }
            idx += 1;
        }
        old = new;
        new = String::new();
    }

    let total: Vec<char> = old.chars().collect();
    let mut letters = total.clone();
    let mut sorted: Vec<(char, u64)> = Vec::new();
    letters.sort_unstable();
    letters.dedup();
    for letter in letters {
        let count = total.iter().filter(|&n| *n == letter).count() as u64;
        sorted.push((letter, count));
    }
    sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    (sorted[sorted.len() - 1].1 - sorted[0].1) as u64
}

fn solve(input: &Input, parts: u8) -> (u64, u64) {
    let runpt1: bool = parts & PART1 != 0;
    let runpt2: bool = parts & PART2 != 0;
    let mut pt1: u64 = 0;
    let mut pt2: u64 = 0;
    if runpt1 {
        pt1 = solve_part1(input);
    }
    if runpt2 {
        pt2 = solve_part2(input);
    }
    (pt1, pt2)
}

fn main() {
    let input = read_input("input/day14.txt");
    let (pt1, pt2) = solve(&input, PART1 | PART2);
    println!("Part1: {:?}", pt1);
    println!("Part2: {:?}", pt2);
}

#[cfg(test)]
mod day14 {
    use crate::*;

    #[test]
    fn part1() {
        let input = read_input("input/sample14.txt");
        let (pt1, _) = solve(&input, PART1);
        assert_eq!(pt1, 1588);
    }
/*
    #[test]
    fn part2() {
        let input = read_input("input/sample14.txt");
        let (_, pt2) = solve(&input, PART2);
        assert_eq!(pt2, 2188189693529);
    }
*/
/*
    #[bench]
    fn bench_day14(b: &mut test::Bencher) {
        b.iter(|| main());
    }
*/
}
