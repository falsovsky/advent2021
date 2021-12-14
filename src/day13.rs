#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use regex::Regex;

const PART1: u8 = 0b01;
const PART2: u8 = 0b10;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Position {
    x: u16,
    y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Position {
        Position { x, y }
    }
}

#[derive(Clone, Debug)]
struct Fold {
    axis: char,
    position: u16,
}

impl Fold {
    pub fn new(axis: char, position: u16) -> Fold {
        Fold { axis, position }
    }
}

#[derive(Debug)]
struct Input {
    positions: Vec<Position>,
    folds: Vec<Fold>,
    height: u16,
    length: u16,
}

impl Input {
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.length {
                if self.positions.contains(&Position::new(x, y)) {
                    print!("#");
                } else {
                    print!(".")
                }
            }
            println!();
        }
        println!();
    }
    pub fn fold(&mut self, axis: char, position: u16) {
        let mut positions: Vec<Position> = Vec::new();
        match axis {
            // horizontal
            'y' => {
                // start
                for y in 0..position {
                    for x in 0..=self.length {
                        let p = Position::new(x, y);
                        if self.positions.contains(&p) {
                            positions.push(p);
                        }
                    }
                }
                // reverse
                for (z, y) in (position..=self.height).enumerate() {
                    for x in 0..=self.length {
                        let mut p = Position::new(x, y);
                        if self.positions.contains(&p) {
                            p.y = position - z as u16;
                            if !positions.contains(&p) {
                                positions.push(p);
                            }
                        }
                    }
                }
                self.height -= position + 1;
            },
            'x' => {
                // start
                for y in 0..=self.height {
                    for x in 0..position {
                        let p = Position::new(x, y);
                        if self.positions.contains(&p) {
                            positions.push(p);
                        }
                    }
                }
                // reverse
                for y in 0..=self.height {
                    for (z, x) in (position..=self.length).enumerate() {
                        let mut p = Position::new(x, y);
                        if self.positions.contains(&p) {
                            p.x = position - z as u16;
                            if !positions.contains(&p) {
                                positions.push(p);
                            }
                        }
                    }
                }
                self.length -= position + 1;
            },
            _ => ()
        };
        self.positions.clear();
        for p in positions {
            self.positions.push(p);
        }
    }
}

fn read_input(filename: &str) -> Input {
    let re_pos = Regex::new(r"(\d+),(\d+)").unwrap();
    let re_fol = Regex::new(r"fold along (x|y)=(\d+)").unwrap();

    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let buffer = BufReader::new(&fp);
    let mut height = 0;
    let mut length = 0;
    let mut positions: Vec<Position> = Vec::new();
    let mut folds: Vec<Fold> = Vec::new();
    for line in buffer.lines() {
        let line_str = match line {
            Ok(value) => value,
            Err(error) => panic!("Could not read anything - {}", error),
        };
        // Position
        if let Some(caps) = re_pos.captures(&line_str) {
            let x: u16 = caps.get(1).unwrap().as_str().parse::<u16>().unwrap();
            let y: u16 = caps.get(2).unwrap().as_str().parse::<u16>().unwrap();
            if y > height { height = y }
            if x > length { length = x }
            positions.push(Position::new(x, y));
        }
        // Fold
        if let Some(caps) = re_fol.captures(&line_str) {
            let axis: char = caps.get(1).unwrap().as_str().chars().next().unwrap();
            let value: u16 = caps.get(2).unwrap().as_str().parse::<u16>().unwrap();
            folds.push(Fold::new(axis, value));
        }
    }
    height += 1;
    length += 1;
    Input { positions, folds , height, length}
}

fn solve_part1(input: &mut Input) -> u64 {
    let fold = &input.folds[0].clone();
    input.fold(fold.axis, fold.position);
    input.positions.len() as u64
}

fn solve_part2(input: &mut Input) {
    let folds = &input.folds.clone();
    for fold in folds {
        input.fold(fold.axis, fold.position);
    }
    input.print();
}

fn solve(input: &mut Input, parts: u8) -> u64 {
    let runpt1: bool = parts & PART1 != 0;
    let runpt2: bool = parts & PART2 != 0;
    let mut pt1: u64 = 0;
    if runpt1 {
        pt1 = solve_part1(input);
    }
    if runpt2 {
        solve_part2(input);
    }
    pt1
}

fn main() {
    let mut input = read_input("input/day13.txt");
    let pt1 = solve(&mut input, PART1);
    println!("Part1: {}", pt1);
    println!("Part2:");
    let mut input = read_input("input/day13.txt");
    solve(&mut input, PART2);
}

#[cfg(test)]
mod day13 {
    use crate::*;

    #[test]
    fn part1() {
        let mut input = read_input("input/sample13.txt");
        let pt1 = solve(&mut input, PART1);
        assert_eq!(pt1, 17);
    }

    #[bench]
    fn bench_day13(b: &mut test::Bencher) {
        b.iter(|| main());
    }
}
