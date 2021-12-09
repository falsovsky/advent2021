#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

const PART1: u8 = 0b01;
const PART2: u8 = 0b10;

    /*
    ['a', 'b', 'c', 'e', 'f', 'g'],      // 0
    ['c', 'f'],                          // 1
    ['a', 'c', 'd', 'e', 'g'],           // 2
    ['a', 'c', 'd', 'f', 'g'],           // 3
    ['b', 'c', 'd', 'f'],                // 4
    ['a', 'b', 'd', 'f', 'g'],           // 5
    ['a', 'b', 'd', 'e', 'f', 'g'],      // 6
    ['a', 'c', 'f'],                     // 7
    ['a', 'b', 'c', 'd', 'e', 'f', 'g'], // 8
    ['a', 'b', 'c', 'd', 'f', 'g']       // 9
    */

#[derive(Debug)]
struct Entry {
    patterns: Vec<String>,
    output: Vec<String>
}

fn read_input(filename: &str) -> Vec<Entry> {
    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let buffer = BufReader::new(&fp);
    let mut input: Vec<Entry> = Vec::new();
    for line in buffer.lines() {
        let line_str = match line {
            Ok(value) => value,
            Err(error) => panic!("Could not read anything - {}", error),
        };
        let values = line_str.split(" | ").collect::<Vec<&str>>();
        // patterns
        let mut p: Vec<String> = Vec::new();
        for pattern in values[0].split(' ') {
            p.push(pattern.to_string());
        }
        // output
        let mut o: Vec<String> = Vec::new();
        for pattern in values[1].split(' ') {
            o.push(pattern.to_string());
        }
        input.push( Entry { patterns: p, output: o });
    }
    input
}

fn solve_part1(input: &[Entry]) -> u32 {
    let mut result: u32 = 0;
    for entry in input {
        for output in &entry.output {
            let len = output.len();
            match len {
                2 | 3 | 4 | 7 => result += 1,
                _ => continue,
            };
        }
    }
    result
}

fn finder(patterns: &[String], wires: &mut HashMap<char, char>, digits: &HashMap<char, Vec<char>>) {
    for ov in patterns {
        let ln = ov.len();

        let mut current: Vec<char> = Vec::new();
        for (_, v) in wires.clone() {
            current.push(v);
        }

        // find a
        if !wires.contains_key(&'a') && digits.contains_key(&'7')
            && digits.contains_key(&'1') {
                let mut a: Vec<char> = digits.get(&'7').unwrap().to_vec();
                let one: Vec<char> = digits.get(&'1').unwrap().to_vec();
                for x in one {
                    a.retain(|e| *e != x);
                }
                if a.len() == 1 {
                    let mut found = false;
                    for m in &current {
                        if a[0] == *m {
                            found = true;
                        }
                    }
                    if !found {
                        wires.insert('a', a[0]);
                    }
                }
            }

        // find g
        if !wires.contains_key(&'g') && wires.contains_key(&'a')
            && digits.contains_key(&'4') && ln == 6 {
                let mut g: Vec<char> = ov.clone().chars().collect();
                let mut a_four: Vec<char> = vec![*wires.get(&'a').unwrap()];
                a_four.extend(digits.get(&'4').unwrap());
                for x in a_four {
                    g.retain(|e| *e != x);
                }
                if g.len() == 1 {
                    let mut found = false;
                    for m in &current {
                        if g[0] == *m {
                            found = true;
                        }
                    }
                    if !found {
                        wires.insert('g', g[0]);
                    }
                }
            }

        // find d
        if !wires.contains_key(&'d') && wires.contains_key(&'a')
            && wires.contains_key(&'g')
            && digits.contains_key(&'1') && ln == 5 {
                let mut d: Vec<char> = ov.clone().chars().collect();
                let mut a_g_one: Vec<char> = vec![
                    *wires.get(&'a').unwrap(),
                    *wires.get(&'g').unwrap()
                ];
                a_g_one.extend(digits.get(&'1').unwrap());
                for x in a_g_one {
                    d.retain(|e| *e != x);
                }
                if d.len() == 1 {
                    let mut found = false;
                    for m in &current {
                        if d[0] == *m {
                            found = true;
                        }
                    }
                    if !found {
                        wires.insert('d', d[0]);
                    }
                }
            }

        // find b
        if !wires.contains_key(&'b') && wires.contains_key(&'a')
            && wires.contains_key(&'d') && wires.contains_key(&'g')
            && digits.contains_key(&'1') && ln == 6 {
                let mut b: Vec<char> = ov.clone().chars().collect();
                let mut a_d_g_one: Vec<char> = vec![
                    *wires.get(&'a').unwrap(),
                    *wires.get(&'d').unwrap(),
                    *wires.get(&'g').unwrap()
                ];
                a_d_g_one.extend(digits.get(&'1').unwrap());
                for x in a_d_g_one {
                    b.retain(|e| *e != x);
                }
                if b.len() == 1 {
                    let mut found = false;
                    for m in &current {
                        if b[0] == *m {
                            found = true;
                        }
                    }
                    if !found {
                        wires.insert('b', b[0]);
                    }
                }
            }

        // find e
        if !wires.contains_key(&'e') && wires.contains_key(&'a')
            && wires.contains_key(&'b') && wires.contains_key(&'g')
            && digits.contains_key(&'1') && ln == 6 {
                let mut e: Vec<char> = ov.clone().chars().collect();
                let mut a_b_g_one: Vec<char> = vec![
                    *wires.get(&'a').unwrap(),
                    *wires.get(&'b').unwrap(),
                    *wires.get(&'g').unwrap()
                ];
                a_b_g_one.extend(digits.get(&'1').unwrap());
                for x in a_b_g_one {
                    e.retain(|e| *e != x);
                }
                if e.len() == 1 {
                    let mut found = false;
                    for m in &current {
                        if e[0] == *m {
                            found = true
                        }
                    }
                    if !found {
                        wires.insert('e', e[0]);
                    }
                }
            }

        // find c
        if !wires.contains_key(&'c') && wires.contains_key(&'a')
            && wires.contains_key(&'d') && wires.contains_key(&'e')
            && wires.contains_key(&'g') && ln == 5 {
                let mut c: Vec<char> = ov.clone().chars().collect();
                let a_d_e_g: Vec<char> = vec![
                    *wires.get(&'a').unwrap(),
                    *wires.get(&'d').unwrap(),
                    *wires.get(&'e').unwrap(),
                    *wires.get(&'g').unwrap(),
                ];
                for x in a_d_e_g {
                    c.retain(|e| *e != x);
                }
                if c.len() == 1 {
                    let mut found = false;
                    for m in &current {
                        if c[0] == *m {
                            found = true;
                        }
                    }
                    if !found {
                        wires.insert('c', c[0]);
                    }
                }
            }

        // find f
        if !wires.contains_key(&'f') && wires.contains_key(&'c') {
            let mut f: Vec<char> = digits.get(&'1').unwrap().to_vec();
            let c: Vec<char> = vec![*wires.get(&'c').unwrap()];
            for x in c {
                f.retain(|e| *e != x);
            }
            if f.len() == 1 {
                let mut found = false;
                for m in &current {
                    if f[0] == *m {
                        found = true;
                    }
                }
                if !found {
                    wires.insert('f', f[0]);
                }
            }
        }
    }
}

fn solve_part2(input: &[Entry]) -> u32 {

    let mut result: u32 = 0;
    for value in input {
        let mut wires: HashMap<char, char> = HashMap::new();
        let mut digits: HashMap<char, Vec<char>> = HashMap::new();
        // Get list of chars for known digits
        for ov in &value.patterns {
            let ln = ov.len();
            // 1
            if ln == 2 {
                digits.insert('1', ov.clone().chars().collect());
            }

            // 4
            if ln == 4 {
                digits.insert('4', ov.clone().chars().collect());
            }

            // 7
            if ln == 3 {
                digits.insert('7', ov.clone().chars().collect());
            }

            // 8
            if ln == 7 {
                digits.insert('8', ov.clone().chars().collect());
            }
        }

        // Run finder until have the definition for the 7 wires
        while wires.len() < 6 {
            finder(&value.patterns, &mut wires, &digits);
        }

        // Add correct order of wires for digits
        // 0
        let val: Vec<char> = vec![
            *wires.get(&'a').unwrap(),
            *wires.get(&'b').unwrap(),
            *wires.get(&'c').unwrap(),
            *wires.get(&'e').unwrap(),
            *wires.get(&'f').unwrap(),
            *wires.get(&'g').unwrap(),
        ];
        digits.insert('0', val);

        // 1
        let val: Vec<char> = vec![
            *wires.get(&'c').unwrap(),
            *wires.get(&'f').unwrap(),
        ];
        digits.insert('1', val);

        // 2
        let val: Vec<char> = vec![
            *wires.get(&'a').unwrap(),
            *wires.get(&'c').unwrap(),
            *wires.get(&'d').unwrap(),
            *wires.get(&'e').unwrap(),
            *wires.get(&'g').unwrap(),
        ];
        digits.insert('2', val);

        // 3
        let val: Vec<char> = vec![
            *wires.get(&'a').unwrap(),
            *wires.get(&'c').unwrap(),
            *wires.get(&'d').unwrap(),
            *wires.get(&'f').unwrap(),
            *wires.get(&'g').unwrap(),
        ];
        digits.insert('3', val);

        // 4
        let val: Vec<char> = vec![
            *wires.get(&'b').unwrap(),
            *wires.get(&'c').unwrap(),
            *wires.get(&'d').unwrap(),
            *wires.get(&'f').unwrap(),
        ];
        digits.insert('4', val);

        // 5
        let val: Vec<char> = vec![
            *wires.get(&'a').unwrap(),
            *wires.get(&'b').unwrap(),
            *wires.get(&'d').unwrap(),
            *wires.get(&'f').unwrap(),
            *wires.get(&'g').unwrap(),
        ];
        digits.insert('5', val);

        // 6
        let val: Vec<char> = vec![
            *wires.get(&'a').unwrap(),
            *wires.get(&'b').unwrap(),
            *wires.get(&'d').unwrap(),
            *wires.get(&'e').unwrap(),
            *wires.get(&'f').unwrap(),
            *wires.get(&'g').unwrap(),
        ];
        digits.insert('6', val);

        // 7
        let val: Vec<char> = vec![
            *wires.get(&'a').unwrap(),
            *wires.get(&'c').unwrap(),
            *wires.get(&'f').unwrap(),
        ];
        digits.insert('7', val);

        // 8
        let val: Vec<char> = vec![
            *wires.get(&'a').unwrap(),
            *wires.get(&'b').unwrap(),
            *wires.get(&'c').unwrap(),
            *wires.get(&'d').unwrap(),
            *wires.get(&'e').unwrap(),
            *wires.get(&'f').unwrap(),
            *wires.get(&'g').unwrap(),
        ];
        digits.insert('8', val);

        // 9
        let val: Vec<char> = vec![
            *wires.get(&'a').unwrap(),
            *wires.get(&'b').unwrap(),
            *wires.get(&'c').unwrap(),
            *wires.get(&'d').unwrap(),
            *wires.get(&'f').unwrap(),
            *wires.get(&'g').unwrap(),
        ];
        digits.insert('9', val);

        // Generate int from chars
        let mut str_num = String::new();
        for ov in &value.output {
            // sort output chars
            let mut sorted_out: Vec<char> = ov.chars().collect();
            sorted_out.sort_unstable();
            for (k, v) in &digits {
                // sort digits
                let mut sorted_d: Vec<char> = v.clone();
                sorted_d.sort_unstable();
                if sorted_out == sorted_d {
                    str_num = format!("{}{}", str_num, k);
                }
            }
        }
        let value = str_num.parse::<u32>().unwrap();
        result += value;
    }

    result
}

fn solve(input: &[Entry], parts: u8) -> (u32, u32) {
    let runpt1: bool = parts & PART1 != 0;
    let runpt2: bool = parts & PART2 != 0;
    let mut pt1: u32 = 0;
    let mut pt2: u32 = 0;
    if runpt1 {
        pt1 = solve_part1(input);
    }
    if runpt2 {
        pt2 = solve_part2(input);
    }
    (pt1, pt2)
}

fn main() {
    let input = read_input("input/day08.txt");
    let (pt1, pt2) = solve(&input, PART1 | PART2);
    println!("Part1: {:?}", pt1);
    println!("Part2: {:?}", pt2);
}

#[cfg(test)]
mod day08 {
    use crate::*;

    #[test]
    fn part1() {
        let input = read_input("input/sample08.txt");
        let (pt1, _) = solve(&input, PART1);
        assert_eq!(pt1, 26);
    }
    #[test]
    fn part2() {
        let input = read_input("input/sample08.txt");
        let (_, pt2) = solve(&input, PART2);
        assert_eq!(pt2, 61229);
    }

    #[bench]
    fn bench_day08(b: &mut test::Bencher) {
        b.iter(|| main());
    }
}
