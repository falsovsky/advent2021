#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const PART1: u8 = 0b01;
const PART2: u8 = 0b10;

type Numbers = Vec<u8>;
type Card = Vec<Numbers>;

fn read_input(filename: &str) -> (Numbers, Vec<Card>) {
    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let buffer = BufReader::new(&fp);
    let mut numbers: Numbers = Vec::new();
    let mut cards: Vec<Card> = Vec::new();
    let mut card: Card = Vec::new();
    let lines = buffer.lines().collect::<Vec<_>>();
    for item in lines[0].as_ref().unwrap().split(',') {
        numbers.push(item.parse::<u8>().expect("Could not convert to u8"));
    }
    let mut idx = 2;
    while idx < lines.len() {
        let mut card_line: Vec<u8> = Vec::new();
        for _ in 0..5 {
            for item in lines[idx].as_ref().unwrap().split(' ') {
                let value: Option<u8> = match item.parse::<u8>() {
                    Ok(value) => Some(value),
                    Err(_) => None,
                };
                if value != None {
                    card_line.push(value.unwrap());
                }
            }
            card.push(card_line.clone());
            card_line = Vec::new();
            idx += 1;
        }
        idx += 1;
        cards.push(card.clone());
        card = Vec::new();
    }
    (numbers, cards)
}

fn check_horizontal(card: &Card, numbers: &Numbers) -> bool {
    for line in card {
        let mut matched = 0;
        for item in line {
            if numbers.contains(item) {
                matched += 1;
            }
        }
        if matched == 5 {
            return true
        }
    }
    false
}

fn check_vertical(card: &Card, numbers: &Numbers) -> bool {
    for column in 0..5 {
        let mut matched = 0;
        for line in 0..5 {
            if numbers.contains(&card[line][column]) {
                matched += 1;
            }
        }
        if matched == 5 {
            return true;
        }
    }
    false
}

fn sum_unmarked(card: &Card, numbers: &Numbers) -> u16 {
    let mut sum: u16 = 0;
    for line in card {
        for item in line {
            if !numbers.contains(item) {
                sum += *item as u16;
            }
        }
    }
    sum
}

fn solve_part1(numbers: &Numbers, cards: &Vec<Card>) -> u32 {
    let mut idx_num = 0;
    let mut idx_card = 0;
    let mut num: Numbers = Vec::new();
    'outer: while idx_num < numbers.len() {
        num = numbers[0..idx_num].to_vec();
        idx_card = 0;
        while idx_card < cards.len() {
            let card = &cards[idx_card];
            let h = check_horizontal(card, &num);
            let v = check_vertical(card, &num);
            if h || v {
                break 'outer;
            }
            idx_card += 1;
        }
        idx_num += 1;
    }

    let called = num[idx_num - 1];
    let sum = sum_unmarked(&cards[idx_card], &num);

    sum as u32 * called as u32
}

fn solve_part2(numbers: &Numbers, cards: &Vec<Card>) -> u32 {

    let mut num: Numbers;
    let mut idx_num;
    let mut idx_card;
    let mut matched: Vec<Card> = Vec::new();
    let mut matched_v: Vec<(u32, u32)> = Vec::new();
    idx_num = 0;
    'outer: while idx_num < numbers.len() {
        num = numbers[0..idx_num].to_vec();
        idx_card = 0;
        while idx_card < cards.len() {
            let card: Card = cards.get(idx_card).unwrap().to_vec();
            if !matched.contains(&card) {
                let h = check_horizontal(&card, &num);
                let v = check_vertical(&card, &num);
                if h || v {
                    matched.push(card);
                    matched_v.push((idx_num.try_into().unwrap(), idx_card.try_into().unwrap()));
                }
                if matched.len() == cards.len() {
                    break 'outer;
                }
            }
            idx_card += 1;
        }
        idx_num += 1;
    }

    let lastcard = matched.last().unwrap();
    let lastidx = matched_v.last().unwrap();
    let nums = numbers[0..lastidx.0 as usize].to_vec();

    let called: u32 = *nums.last().unwrap() as u32;
    let sum = sum_unmarked(lastcard, &numbers[0..lastidx.0 as usize].to_vec());

    sum as u32 * called as u32

}

fn solve(numbers: &Numbers, cards: &Vec<Card>, parts: u8) -> (u32, u32) {
    let runpt1: bool = parts & PART1 != 0;
    let runpt2: bool = parts & PART2 != 0;
    let mut pt1: u32 = 0;
    let mut pt2: u32 = 0;
    if runpt1 {
        pt1 = solve_part1(numbers, cards);
    }
    if runpt2 {
        pt2 = solve_part2(numbers, cards);
    }
    (pt1, pt2)
}

fn main() {
    let (numbers, cards) = read_input("input/day04.txt");
    let (pt1, pt2) = solve(&numbers, &cards, PART1 | PART2);
    println!("Part1: {:?}", pt1);
    println!("Part2: {:?}", pt2);
}

#[cfg(test)]
mod day04 {
    use crate::*;

    #[test]
    fn part1() {
        let (numbers, cards) = read_input("input/sample04.txt");
        let (pt1, _) = solve(&numbers, &cards, PART1);
        assert_eq!(pt1, 4512);
    }

    #[test]
    fn part2() {
        let (numbers, cards) = read_input("input/sample04.txt");
        let (_, pt2) = solve(&numbers, &cards, PART2);
        assert_eq!(pt2, 1924);
    }

    #[bench]
    fn bench_day04(b: &mut test::Bencher) {
        b.iter(|| main());
    }
}
