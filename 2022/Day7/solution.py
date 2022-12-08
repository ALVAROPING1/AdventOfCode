import heapq
from typing import Callable

def part1(_input: str) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		lines = file.readlines()
	total_sum = TotalSum()
	calculate_dir_size(lines, len(lines), 1, calculate_dir_size_sum, total_sum)
	return total_sum.value

def part2(_input: str) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		lines = file.readlines()
	priority_queue = []
	used_size, _ = calculate_dir_size(lines, len(lines), 1, add_dir_size_queue, priority_queue)
	required_size = 30_000_000 - (70_000_000 - used_size)
	sorted_sizes = sorted(priority_queue)
	return binary_search(sorted_sizes, required_size, 0, len(sorted_sizes) - 1)

def binary_search(_list: list[int], value: int, start: int, end: int) -> int:
	middle = start + (end - start) // 2
	current_value = _list[middle]
	if current_value == value or start == end:
		return current_value
	if current_value > value:
		return binary_search(_list, value, start, middle - 1)
	return binary_search(_list, value, middle + 1, end)

class TotalSum():
	def __init__(self, value: int = 0) -> None:
		self.value = value

def calculate_dir_size_sum(current_dir_size: int, total_sum: TotalSum) -> None:
	if current_dir_size <= 100_000:
		total_sum.value += current_dir_size

def add_dir_size_queue(current_dir_size: int, queue: list[int]) -> None:
	heapq.heappush(queue, current_dir_size)

def calculate_dir_size(lines: list[str], len_lines: int, index: int, current_dir_size_function: Callable[[int, object], None], state_object: object) -> tuple[int, int]:
	current_dir_size = 0
	index += 1
	while True:
		if index >= len_lines:
			current_dir_size_function(current_dir_size, state_object)
			return current_dir_size, index
		line = lines[index]
		index += 1
		if line[0] == "$":
			if line[5] == ".":
				current_dir_size_function(current_dir_size, state_object)
				return current_dir_size, index
			dir_size, index = calculate_dir_size(lines, len_lines, index, current_dir_size_function, state_object)
			current_dir_size += dir_size
		else:
			first_word = line.split(" ", maxsplit=1)[0]
			if first_word[0] != "d":
				current_dir_size += int(first_word)

_input = "./2022/Day7/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
