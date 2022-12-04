def part1(_input: str) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		return sum(rangeIsSubset(*map(int, line.replace(",", "-").split("-"))) for line in file.readlines())

def part2(_input: str) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		return sum(rangeOverlap(*map(int, line.replace(",", "-").split("-"))) for line in file.readlines())

def rangeIsSubset(start1: int, end1: int, start2: int, end2: int) -> bool:
	return start1 == start2 or end1 == end2 or (start1 > start2) == (end1 < end2)

def rangeOverlap(start1: int, end1: int, start2: int, end2: int) -> bool:
	return start1 <= end2 and start2 <= end1

_input = "./2022/Day4/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
