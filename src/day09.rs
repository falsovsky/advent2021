#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

const PART1: u8 = 0b01;
const PART2: u8 = 0b10;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i16,
    y: i16,
}

type Row = HashMap<Point, i16>;

fn read_input(filename: &str) -> Row {
    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let buffer = BufReader::new(&fp);
    let mut input: Row = HashMap::new();
    for (x, line) in buffer.lines().enumerate() {
        let line_str = match line {
            Ok(value) => value,
            Err(error) => panic!("Could not read anything - {}", error),
        };
        let chars = line_str.chars();
        for (y, char) in chars.into_iter().enumerate() {
            input.insert(
                Point {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap()
                },
                char.to_digit(10).unwrap().try_into().unwrap()
            );
        }
    }
    input
}

fn get_dimensions(input: &Row) -> (u8, u8) {
    let mut h = 0;
    let mut w = 0;
    while let Some(_value) = input.get(&Point{ x: 0, y: h }) {
        h += 1;
    }
    while let Some(_value) = input.get(&Point{ x: w, y: 0 }) {
        w += 1;
    }
    (h as u8, w as u8)
}

fn is_adjacent(input: &Row, position: &Point) -> bool {
    let top_pos = Point { x: position.x, y: position.y - 1 };
    let bottom_pos = Point { x: position.x, y: position.y + 1 };
    let left_pos = Point { x: position.x - 1, y: position.y };
    let right_pos = Point { x: position.x + 1, y: position.y };
    let height = *input.get(position).unwrap();
    let mut values: Vec<i16> = vec![
        *input.get(&top_pos).unwrap_or(&-1),
        *input.get(&bottom_pos).unwrap_or(&-1),
        *input.get(&left_pos).unwrap_or(&-1),
        *input.get(&right_pos).unwrap_or(&-1)
    ];
    values.retain(|v| *v != -1);
    let mut result = true;
    for value in &values {
        if value <= &height {
            result = false;
        }
    }
    result
}

fn solve_part1(input: &Row) -> i16 {
    let (h , w) = get_dimensions(input);
    let mut total = 0;
    for x in 0..w as i16{
        for y in 0..h as i16 {
            let p = Point { x, y };
            let v = input.get(&p).unwrap();
            let valid = is_adjacent(input, &p);
            if valid {
                total += v + 1;
            }
        }
    }
    total
}

fn is_increment(input: &Row, position: Point, values: &mut Vec<Point>) {
    let value = *input.get(&position).unwrap();
    if !values.contains(&position) {
        values.push(position.clone());
    }
    let pos = vec![
        Point { x: position.x, y: position.y - 1 }, // top
        Point { x: position.x, y: position.y + 1 }, // bottom
        Point { x: position.x - 1, y: position.y }, // left
        Point { x: position.x + 1, y: position.y }  // right
    ];
    for p in pos {
        let v = *input.get(&p).unwrap_or(&-1);
        if v >= 0 && v > value && v < 9 {
            is_increment(input, p, values);
        }
    }
}

fn solve_part2(input: &Row) -> i32 {
    // get same positions as part 1
    let (h , w) = get_dimensions(input);
    let mut points: Vec<Point> = Vec::new();
    for x in 0..w as i16{
        for y in 0..h as i16 {
            let p = Point { x, y };
            if is_adjacent(input, &p) {
                points.push(p);
            }
        }
    }
    let mut total: i32 = 1;
    // get the len of each basin
    let mut sizes: Vec<i16> = Vec::new();
    for p in points {
        let mut values: Vec<Point> = Vec::new();
        is_increment(input, p, &mut values);
        sizes.push(values.len() as i16);
    }
    // sort, reverse and get first 3
    sizes.sort_unstable();
    sizes.reverse();
    sizes = sizes[0..3].to_vec();
    for s in sizes {
        total *= s as i32;
    }

    total
}

fn solve(input: &Row, parts: u8) -> (i16, i32) {
    let runpt1: bool = parts & PART1 != 0;
    let runpt2: bool = parts & PART2 != 0;
    let mut pt1: i16 = 0;
    let mut pt2: i32 = 0;
    if runpt1 {
        pt1 = solve_part1(input);
    }
    if runpt2 {
        pt2 = solve_part2(input);
    }
    (pt1, pt2)
}

fn main() {
    let input = read_input("input/day09.txt");
    let (pt1, pt2) = solve(&input, PART1 | PART2);
    println!("Part1: {:?}", pt1);
    println!("Part2: {:?}", pt2);
}

#[cfg(test)]
mod day09 {
    use crate::*;

    #[test]
    fn part1() {
        let input = read_input("input/sample09.txt");
        let (pt1, _) = solve(&input, PART1);
        assert_eq!(pt1, 15);
    }

    #[test]
    fn part2() {
        let input = read_input("input/sample09.txt");
        let (_, pt2) = solve(&input, PART2);
        assert_eq!(pt2, 1134);
    }

    #[bench]
    fn bench_day09(b: &mut test::Bencher) {
        b.iter(|| main());
    }
}
