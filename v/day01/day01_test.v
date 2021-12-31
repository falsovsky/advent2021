module main

const test_input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]

fn test_solve_part1() {
	result := solve_part1(test_input)
	assert result == 7
}

fn test_solve_part2() {
	result := solve_part2(test_input)
	assert result == 5
}
