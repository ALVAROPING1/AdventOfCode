def part1(_input: str) -> int:
	return solve(_input, 4)

def part2(_input: str) -> int:
	return solve(_input, 14)

def solve(_input: str, max_size: int) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		stream = file.read()[:-1]
		window_start = 0
		last_seen = {char: 0 for char in "abcdefghijklmnopqrstuvwxyz"}
		for index, char in enumerate(stream):
			last_seen_index = last_seen[char]
			if last_seen_index > window_start:
				window_start = last_seen_index
			elif index + 1 - window_start >= max_size:
				return index + 1
			last_seen[char] = index + 1

_input = "./2022/Day6/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
