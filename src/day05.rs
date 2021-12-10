#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

const PART1: u8 = 0b01;
const PART2: u8 = 0b10;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32
}
#[derive(Debug, Copy, Clone)]
struct Line {
    start: Point,
    end: Point
}

fn read_input(filename: &str) -> Vec<Line> {
    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let buffer = BufReader::new(&fp);
    let mut lines: Vec<Line> = Vec::new();
    for line in buffer.lines() {
        let line_str = match line {
            Ok(value) => value,
            Err(error) => panic!("Could not read anything - {}", error),
        };
        let mut points: Vec<Point> = Vec::new();
        for points_str in line_str.split(" -> ") {
            let mut values: Vec<i32> = Vec::new();
            for value in points_str.split(',') {
                values.push(value.parse::<i32>().expect("Could not convert to i32"))
            }
            points.push(Point { x: values[0], y: values[1] });
        }
        lines.push(Line { start: points[0], end: points[1] });
    }
    lines
}

fn set_or_increment(visited: &mut HashMap<Point, u16>, point: &Point) {
    if visited.contains_key(point) {
        let value: u16 = *visited.get(point).unwrap();
        visited.insert(*point, value + 1);
    } else {
        visited.insert(*point, 1);
    }
}

fn h_or_v_lines(lines: &[Line], mut visited: &mut HashMap<Point, u16>) {
    for line in lines {
        if line.start.x != line.end.x && line.start.y != line.end.y {
            continue;
        }
        // x1 < x2 && y1 == y2
        if line.start.x < line.end.x && line.start.y == line.end.y {
            for x in line.start.x..=line.end.x {
                let point: Point = Point { x, y: line.start.y };
                set_or_increment(&mut visited, &point);
            }
        }
        // x1 > x2 && y1 == y2
        if line.start.x > line.end.x && line.start.y == line.end.y {
            for x in line.end.x..=line.start.x {
                let point: Point = Point { x, y: line.start.y };
                set_or_increment(&mut visited, &point);
            }
        }
        // x1 == x2 && y1 < y2
        if line.start.x == line.end.x && line.start.y < line.end.y {
            for y in line.start.y..=line.end.y {
                let point: Point = Point { x: line.start.x, y };
                set_or_increment(&mut visited, &point);
            }
        }
        // x1 == x2 && y1 > y2
        if line.start.x == line.end.x && line.start.y > line.end.y {
            for y in line.end.y..=line.start.y {
                let point: Point = Point { x: line.start.x, y };
                set_or_increment(&mut visited, &point);
            }
        }
    }
}

fn diagonal_lines(lines: &[Line], mut visited: &mut HashMap<Point, u16>) {
    for line in lines {
        if line.start.x == line.end.x || line.start.y == line.end.y {
            continue;
        }
        let mut position: Point = Point { x: line.start.x, y:line.start.y };
        let finish: Point = Point { x: line.end.x, y:line.end.y };
        // x1 < x2 && y1 < y2
        if line.start.x < line.end.x && line.start.y < line.end.y {
            while position.x <= finish.x && position.y <= finish.y {
                set_or_increment(&mut visited, &position);
                position.x += 1;
                position.y += 1;
            }
        }
        // x1 > x2 && y1 > y2
        if line.start.x > line.end.x && line.start.y > line.end.y {
            while position.x >= finish.x && position.y >= finish.y {
                set_or_increment(&mut visited, &position);
                position.x -= 1;
                position.y -= 1;
            }
        }
        // x1 > x2 && y1 < y2
        if line.start.x > line.end.x && line.start.y < line.end.y {
            while position.x >= finish.x && position.y <= finish.y {
                set_or_increment(&mut visited, &position);
                position.x -= 1;
                position.y += 1;
            }
        }
        // x1 < x2 && y1 > y2
        if line.start.x < line.end.x && line.start.y > line.end.y {
            while position.x <= finish.x && position.y >= finish.y {
                set_or_increment(&mut visited, &position);
                position.x += 1;
                position.y -= 1;
            }
        }
    }
}

fn solve_part1(lines: &[Line]) -> u32 {
    let mut visited: HashMap<Point, u16> = HashMap::new();
    h_or_v_lines(lines, &mut visited);
    let mut danger = 0;
    for value in visited.values() {
        if value > &danger {
            danger = *value;
        }
    }
    visited.retain(|_key, value| {
        value > &mut 1
    });
    visited.len().try_into().unwrap()
}

fn solve_part2(lines: &[Line]) -> u32 {
    let mut visited: HashMap<Point, u16> = HashMap::new();
    h_or_v_lines(lines, &mut visited);
    diagonal_lines(lines, &mut visited);
    let mut danger = 0;
    for value in visited.values() {
        if value > &danger {
            danger = *value;
        }
    }
    visited.retain(|_key, value| {
        value > &mut 1
    });
    visited.len().try_into().unwrap()
}

fn solve(lines: &[Line], parts: u8) -> (u32, u32) {
    let runpt1: bool = parts & PART1 != 0;
    let runpt2: bool = parts & PART2 != 0;
    let mut pt1: u32 = 0;
    let mut pt2: u32 = 0;
    if runpt1 {
        pt1 = solve_part1(lines);
    }
    if runpt2 {
        pt2 = solve_part2(lines);
    }
    (pt1, pt2)
}

fn main() {
    let lines = read_input("input/day05.txt");
    let (pt1, pt2) = solve(&lines, PART1 | PART2);
    println!("Part1: {:?}", pt1);
    println!("Part2: {:?}", pt2);
}

#[cfg(test)]
mod day05 {
    use crate::*;

    #[test]
    fn part1() {
        let lines = read_input("input/sample05.txt");
        let (pt1, _) = solve(&lines, PART1);
        assert_eq!(pt1, 5);
    }

    #[test]
    fn part2() {
        let lines = read_input("input/sample05.txt");
        let (_, pt2) = solve(&lines, PART2);
        assert_eq!(pt2, 12);
    }

    #[bench]
    fn bench_day05(b: &mut test::Bencher) {
        b.iter(|| main());
    }
}
