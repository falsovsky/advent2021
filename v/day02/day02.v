module main

import os

struct Command {
	direction string
	unit      int
}

struct Point {
mut:
	x int
	y int
}

fn read_input(filename string) []Command {
	mut values := []Command{}
	lines := os.read_lines('input/' + filename) or { panic('Could not read file') }
	for line in lines {
		value := line.split(' ')
		values << Command{value[0], value[1].int()}
	}
	return values
}

fn solve_part1(input []Command) int {
	mut position := Point{0, 0}
	for command in input {
		match command.direction {
			'forward' { position.y += command.unit }
			'down' { position.x += command.unit }
			'up' { position.x -= command.unit }
			else { panic('invalid instruction $command.direction') }
		}
	}
	return position.x * position.y
}

fn solve_part2(input []Command) int {
	mut position := Point{0, 0}
	mut aim := 0
	for command in input {
		match command.direction {
			'forward' {
				position.x += aim * command.unit
				position.y += command.unit
			}
			'down' {
				aim += command.unit
			}
			'up' {
				aim -= command.unit
			}
			else {
				panic('invalid instruction $command.direction')
			}
		}
	}
	return position.x * position.y
}

fn main() {
	input := read_input('day02.txt')
	part1 := solve_part1(input)
	println('Part1: $part1')
	part2 := solve_part2(input)
	println('Part2: $part2')
}
