module main

import os

fn read_input(filename string) []int {
	mut values := []int{}
	lines := os.read_lines('input/' + filename) or { panic('Could not read file') }
	for line in lines {
		values << line.int()
	}
	return values
}

fn solve_part1(input []int) int {
	mut value, mut last := 0, 0
	for current in input {
		if last > 0 && last < current {
			value++
		}
		last = current
	}
	return value
}

fn solve_part2(input []int) int {
	mut value, mut last, mut pc := 0, 0, 0
	for pc < input.len - 2 {
		current := input[pc] + input[pc + 1] + input[pc + 2]
		if last > 0 && last < current {
			value++
		}
		last = current
		pc++
	}
	return value
}

fn main() {
	input := read_input('day01.txt')
	part1 := solve_part1(input)
	println('Part1: $part1')
	part2 := solve_part2(input)
	println('Part2: $part2')
}
