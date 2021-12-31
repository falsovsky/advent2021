module main

const test_input = [
	Command{'forward', 5},
	Command{'down', 5},
	Command{'forward', 8},
	Command{'up', 3},
	Command{'down', 8},
	Command{'forward', 2},
]

fn test_solve_part1() {
	result := solve_part1(test_input)
	assert result == 150
}

fn test_solve_part2() {
	result := solve_part2(test_input)
	assert result == 900
}
