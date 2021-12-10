#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const PART1: u8 = 0b01;
const PART2: u8 = 0b10;

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let buffer = BufReader::new(&fp);
    let mut input: Vec<Vec<char>> = Vec::new();
    for line in buffer.lines() {
        let line_str = match line {
            Ok(value) => value,
            Err(error) => panic!("Could not read anything - {}", error),
        };
        let chars = line_str.chars();
        input.push(chars.collect());
    }
    input
}

fn oposite(c: char) -> char {
    let result: char = match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        _ => '#'
    };
    result
}

fn closes(input: &[char], position: usize, stack: &mut Vec<char>, extra: &mut Vec<char>) {
    let open: Vec<char> = vec!['(', '[', '{', '<'];
    let close: Vec<char> = vec![')', ']', '}', '>'];
    // If its incomplete, clean the stack
    // and add the missing ones to extra
    if position >= input.len() {
        while !stack.is_empty() {
            let item = stack.pop().unwrap();
            extra.push(oposite(item));
        }
        return;
    }
    let c = input[position];
    // if its a opening one add to stack
    if open.contains(&c) {
        stack.push(c);
        closes(input, position + 1, stack, extra);
    }
    // if its a closing one and the stack is not empty
    if close.contains(&c) && !stack.is_empty() {
        // pop the last item of the stack
        let last = stack.pop().unwrap();
        // check if its the oposite
        if last == oposite(c) {
            closes(input, position + 1, stack, extra);
        } else {
            // invalid, add the culprit to the end of the stack
            stack.push(c);
        }
    }
}

fn solve_part1(input: &[Vec<char>]) -> u64 {
    let mut total = 0;
    for item in input {
        let mut stack: Vec<char> = Vec::new();
        closes(item, 0, &mut stack, &mut vec![]);
        if !stack.is_empty() {
            let fail = stack.pop().unwrap();
            match fail {
                ')' => { total += 3 },
                ']' => { total += 57 },
                '}' => { total += 1197 },
                '>' => { total += 25137 },
                _ => panic!("INVALID {}", fail)
            }
        }
    }
    total
}

fn get_points(values: &[char]) -> u64 {
    let mut total: u64 = 0;
    for v in values {
        total *= 5;
        total += match v {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("INVALID {}", v)
        };
    }
    total
}

fn solve_part2(input: &[Vec<char>]) -> u64 {
    let mut points: Vec<u64> = Vec::new();
    for item in input {
        let mut stack: Vec<char> = Vec::new();
        let mut extra: Vec<char> = Vec::new();
        closes(item, 0, &mut stack, &mut extra);
        if !extra.is_empty() {
            points.push(get_points(&extra));
        }
    }
    points.sort_unstable();
    points[points.len() / 2]
}

fn solve(input: &[Vec<char>], parts: u8) -> (u64, u64) {
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
    let input = read_input("input/day10.txt");
    let (pt1, pt2) = solve(&input, PART1 | PART2);
    println!("Part1: {:?}", pt1);
    println!("Part2: {:?}", pt2);
}

#[cfg(test)]
mod day10 {
    use crate::*;

    #[test]
    fn part1() {
        let input = read_input("input/sample10.txt");
        let (pt1, _) = solve(&input, PART1);
        assert_eq!(pt1, 26397);
    }

    #[test]
    fn part2() {
        let input = read_input("input/sample10.txt");
        let (_, pt2) = solve(&input, PART2);
        assert_eq!(pt2, 288957);
    }

    #[bench]
    fn bench_day10(b: &mut test::Bencher) {
        b.iter(|| main());
    }
}
