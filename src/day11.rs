#![feature(test)]

extern crate test;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const PART1: u8 = 0b01;
const PART2: u8 = 0b10;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Position {
    x: i8,
    y: i8
}

#[derive(Debug, Copy, Clone)]
struct Item {
    value: u8,
    exploded: bool
}

impl Position {
    pub fn new(x: i8, y: i8) -> Position {
        Position { x, y }
    }
    pub fn get_tuple(&self) -> (i8, i8) {
        (self.x, self.y)
    }
}

impl Item {
    pub fn new(value: u8, exploded: bool) -> Item {
        Item { value, exploded }
    }
    pub fn get_tuple(&self) -> (u8, bool) {
        (self.value, self.exploded)
    }
}

fn read_input(filename: &str) -> HashMap<Position, Item> {
    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let buffer = BufReader::new(&fp);
    let mut input: HashMap<Position, Item> = HashMap::new();
    for (x, line) in buffer.lines().enumerate() {
        let line_str = match line {
            Ok(value) => value,
            Err(error) => panic!("Could not read anything - {}", error),
        };
        for (y, value) in line_str.chars().enumerate() {
            let pos = Position::new(x as i8, y as i8);
            let digit = value.to_digit(10).unwrap();
            let item = Item::new(digit as u8, false);
            input.insert(pos, item);
        }
    }
    input
}

fn get_size(input: &HashMap<Position, Item>) -> usize {
    let mut x: usize = 0;
    loop {
        let default = Item::new(255, false);
        let position = Position::new(x as i8, 0);
        let item = input.get(&position).unwrap_or(&default);
        let (value, _flashed) = item.get_tuple();
        if value == 255 {
            break;
        }
        x += 1;
    }
    x
}

fn flash(input: &mut HashMap<Position, Item>, position: Position) {
    let (x, y) = position.get_tuple();
    let positions = vec![
        Position::new(x, y - 1),     // top
        Position::new(x, y + 1),     // bottom
        Position::new(x - 1, y),     // left
        Position::new(x + 1, y),     // right
        Position::new(x - 1, y - 1), // top left
        Position::new(x + 1, y - 1), // top right
        Position::new(x - 1, y + 1), // bottom left
        Position::new(x + 1, y + 1), // bottom right
    ];
    input.insert(position, Item::new(0, true));
    for adjacent in positions {
        let default = Item::new(255, false);
        let item = input.get(&adjacent).unwrap_or(&default);
        let (mut value, flashed) = item.get_tuple();
        if value != 255 {
            if value < 9 {
                value += 1;
                input.insert(adjacent, Item::new(value, flashed));
            } else if !flashed {
                flash(input, adjacent);
            } else {
                return;
            }
        }
    }
}

fn run_steps(input: &HashMap<Position, Item>, steps: usize, parts: u8) -> u64 {
    let runpt2: bool = parts & PART2 != 0;
    let mut flashes: u64 = 0;
    let size = get_size(input);
    let mut myinput: HashMap<Position, Item> = HashMap::new();
    for (p, i) in input {
        myinput.insert(*p, *i);
    }
    for step in 0..steps {
        // Increment
        for x in 0..size {
            for y in 0..size {
                let pos = Position::new(x as i8, y as i8);
                let item = *myinput.get(&pos).unwrap();
                let val = item.value + 1;
                myinput.insert(pos, Item::new(val, item.exploded));
            }
        }
        // Check for explosions
        for x in 0..size {
            for y in 0..size {
                let pos = Position::new(x as i8, y as i8);
                let item = myinput.get(&pos).unwrap();
                if item.value == 10 {
                    flash(&mut myinput, pos);
                }
            }
        }
        // Set exploded to 0
        let mut explosions = 0;
        for x in 0..size {
            for y in 0..size {
                let pos = Position::new(x as i8, y as i8);
                let item = myinput.get(&pos).unwrap();
                if item.exploded {
                    myinput.insert(pos, Item::new(0, false));
                    flashes += 1;
                    explosions += 1
                }
            }
        }
        if explosions == 100 && runpt2 {
            return step as u64 + 1;
        }
    }
    flashes
}

fn solve_part1(input: &HashMap<Position, Item>) -> u64 {
    run_steps(input, 100, PART1)
}

fn solve_part2(input: &HashMap<Position, Item>) -> u64 {
    run_steps(input,  usize::max_value(), PART2)
}

fn solve(input: &HashMap<Position, Item>, parts: u8) -> (u64, u64) {
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
    let input = read_input("input/day11.txt");
    let (pt1, pt2) = solve(&input, PART1 | PART2);
    println!("Part1: {:?}", pt1);
    println!("Part2: {:?}", pt2);
}

#[cfg(test)]
mod day11 {
    use crate::*;

    #[test]
    fn part1() {
        let input = read_input("input/sample11.txt");
        let (pt1, _) = solve(&input, PART1);
        assert_eq!(pt1, 1656);
    }

    #[test]
    fn part2() {
        let input = read_input("input/sample11.txt");
        let (_, pt2) = solve(&input, PART2);
        assert_eq!(pt2, 195);
    }

    #[bench]
    fn bench_day11(b: &mut test::Bencher) {
        b.iter(|| main());
    }
}
