from heapq import nlargest

def part1(_input: str) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		return max(map(sum, (map(int, x.split("\n")) for x in file.read()[:-1].split("\n\n"))))

def part2(_input: str) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		return sum(nlargest(3, map(sum, (map(int, x.split("\n")) for x in file.read()[:-1].split("\n\n")))))

_input = "./2022/Day1/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
