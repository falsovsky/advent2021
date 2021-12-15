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
    rules: HashMap<(char, char), char>
}

fn read_input(filename: &str) -> Input {
    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let buffer = BufReader::new(&fp);
    let mut template: String = String::new();
    let mut rules: HashMap<(char, char), char> = HashMap::new();
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
        let mut pair_chars = values[0].chars();
        let pair: (char, char) = (pair_chars.next().unwrap(), pair_chars.next().unwrap());
        let mut value_chars = values[1].chars();
        let value: char = value_chars.next().unwrap();
        rules.insert(pair, value);

    }
    Input { template, rules }
}

fn parse_template(input: &Input) -> HashMap<(char, char), u64> {
    let mut old: HashMap<(char, char), u64> = HashMap::new();
    let mut idx = 0;
    while idx < input.template.len() - 1 {
        let pair_c = input.template[idx..=idx+1].to_string();
        let mut pair_chars = pair_c.chars();
        let pair: (char, char) = (pair_chars.next().unwrap(), pair_chars.next().unwrap());
        if input.rules.contains_key(&pair) {
            let mut current: u64 = *old.get(&pair).unwrap_or(&0);
            current += 1;
            old.insert(pair, current);
        }
        idx += 1;
    }
    old
}

fn do_steps(input: &Input, steps: u8) -> HashMap<(char, char), u64> {
    let mut old = parse_template(input);
    let mut new: HashMap<(char, char), u64> = HashMap::new();
    for _ in 0..steps {
        for (k, v) in &old {
            let middle: char = *input.rules.get(k).unwrap();
            let start: (char, char) = (k.0 , middle);
            let end: (char, char) = (middle, k.1);
            if input.rules.contains_key(&start) {
                let mut current: u64 = *new.get(&start).unwrap_or(&0);
                current += v;
                new.insert(start, current);
            }
            if input.rules.contains_key(&end) {
                let mut current: u64 = *new.get(&end).unwrap_or(&0);
                current += v;
                new.insert(end, current);
            }
        }
        old = new;
        new = HashMap::new();
    }
    old
}

fn get_result(input: &Input, polymers: &HashMap<(char, char), u64>) -> u64{
    let mut counter: HashMap<char, u64> = HashMap::new();
    for (k, v) in polymers {
        let ch1: char = k.0;
        let mut val: u64 = *counter.get(&ch1).unwrap_or(&0);
        val += v;
        counter.insert(ch1, val);
    }
    // Add last char from template
    let last: char = input.template.chars().last().unwrap();
    let mut val: u64 = *counter.get(&last).unwrap_or(&0);
    val += 1;
    counter.insert(last, val);

    let mut sorted: Vec<(char, u64)> = Vec::new();
    for (k, v) in counter {
        sorted.push((k, v));
    }
    sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    (sorted[sorted.len() - 1].1 - sorted[0].1) as u64
}

fn solve_part1(input: &Input) -> u64 {
    let result = do_steps(input, 10);
    get_result(input, &result)
}

fn solve_part2(input: &Input) -> u64 {
    let result = do_steps(input, 40);
    get_result(input, &result)
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

    #[test]
    fn part2() {
        let input = read_input("input/sample14.txt");
        let (_, pt2) = solve(&input, PART2);
        assert_eq!(pt2, 2188189693529);
    }

    #[bench]
    fn bench_day14(b: &mut test::Bencher) {
        b.iter(|| main());
    }
}
