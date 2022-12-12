from heapq import nlargest
from operator import mul
from functools import reduce

def part1(_input: str) -> int:
	return solve(_input, 20, True)

def part2(_input: str) -> int:
	return solve(_input, 10000, False)

def solve(_input: str, rounds: int, reduce_worry: bool) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		monkey_list: list[Monkey] = []
		for monkey in file.read().split("\n\n"):
			lines = monkey.split("\n")
			monkey_list.append(Monkey(
				list(map(int, lines[1][18:].split(", "))),
				lines[2][23],
				lines[2][25:],
				reduce_worry,
				int(lines[3][21:]),
				(int(lines[5][30]), int(lines[4][29]))
			))
		test_prod = reduce(mul, (monkey.test_value for monkey in monkey_list))
		for _ in range(rounds):
			for monkey in monkey_list:
				for item, target in monkey.evaluate_items():
					monkey_list[target].items.append(item % test_prod)
	return reduce(mul, nlargest(2, (monkey.inspected_items_count for monkey in monkey_list)))

class Monkey:
	def __init__(self, items: list[int], operation_type: str, operation_value: str, reduce_worry: bool, test_value: int, test_result: tuple[int, int]) -> None:
		self.inspected_items_count = 0
		self.items = items
		if operation_type == "+":
			if reduce_worry:
				self.operation = lambda x: (x + int(operation_value))//3
			else:
				self.operation = lambda x: x + int(operation_value)
		elif operation_value[0] == "o":
			if reduce_worry:
				self.operation = lambda x: (x ** 2)//3
			else:
				self.operation = lambda x: x ** 2
		else:
			if reduce_worry:
				self.operation = lambda x: (x * int(operation_value))//3
			else:
				self.operation = lambda x: x * int(operation_value)
		self.test_value = test_value
		self.test_result = test_result
	
	def evaluate_items(self) -> tuple[int, int]:
		for item in self.items:
			item_value = self.operation(item)
			yield (item_value, self.test_result[item_value % self.test_value == 0])
		self.inspected_items_count += len(self.items)
		self.items = []

_input = "./2022/Day11/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
