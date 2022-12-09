from operator import add

def part1(_input: str) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		trees = tuple(tuple(map(int, tuple(x[:-1]))) for x in file.readlines())
	len_line = len(trees[0])
	len_column = len(trees)
	visible_matrix = [[False for _ in range(len_line)] for _ in range(len_column)]
	return sum(sum((
		check_visible(trees, visible_matrix, True, range(len_line), index),
		check_visible(trees, visible_matrix, True, range(len_line-1, -1, -1), index),
		check_visible(trees, visible_matrix, False, range(len_column), index),
		check_visible(trees, visible_matrix, False, range(len_column-1, -1, -1), index)
	)) for index in range(len_column))

def part2(_input: str) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		trees = tuple(tuple(map(int, tuple(x[1:-2]))) for x in file.readlines()[1:-1])
	len_line = len(trees[0])
	len_column = len(trees)
	scenic_score_matrix = [[1 for _ in range(len_line)] for _ in range(len_column)]
	for index in range(len_column):
		check_visible_amount(trees, scenic_score_matrix, True, range(len_line), index)
		check_visible_amount(trees, scenic_score_matrix, True, range(len_line-1, -1, -1), index)
		check_visible_amount(trees, scenic_score_matrix, False, range(len_column), index)
		check_visible_amount(trees, scenic_score_matrix, False, range(len_column-1, -1, -1), index)
	return max(max(line) for line in scenic_score_matrix)

def check_visible(trees: tuple[tuple[int]], visible_matrix: list[list[int]], line: bool, range_object: range, fixed: int) -> int:
	total_visible = 0
	current_tallest = -1
	for index in range_object:
		current_tree = get_matrix_value(trees, fixed, index, line)
		if current_tree > current_tallest:
			current_tallest = current_tree
			if not get_matrix_value(visible_matrix, fixed, index, line):
				set_matrix_value(visible_matrix, fixed, index, line, True)
				total_visible += 1
			if current_tree == 9:
				break
	return total_visible

def check_visible_amount(trees: tuple[tuple[int]], scenic_score_matrix: list[list[int]], line: bool, range_object: range, fixed: int):
	# amount_of_tress[i] = #trees < i
	amount_of_trees = [0 for _ in range(10)]
	for index in range_object:
		current_tree = get_matrix_value(trees, fixed, index, line)
		multiply_matrix_value(scenic_score_matrix, fixed, index, line, amount_of_trees[current_tree] + 1)
		amount_of_trees = list(map(add, amount_of_trees, (current_tree < x for x in range(10))))
		amount_of_trees[:current_tree+1] = (0 for _ in range(current_tree+1))

def get_matrix_value(matrix: list[list], fixed: int, index: int, line: bool) -> int:
	return matrix[fixed][index] if line else matrix[index][fixed]

def set_matrix_value(matrix: list[list], fixed: int, index: int, line: bool, value):
	if line: matrix[fixed][index] = value
	else: matrix[index][fixed] = value

def multiply_matrix_value(matrix: list[list], fixed: int, index: int, line: bool, value):
	if line: matrix[fixed][index] *= value
	else: matrix[index][fixed] *= value


_input = "./2022/Day8/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
