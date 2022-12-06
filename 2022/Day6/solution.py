def part1(_input: str) -> int:
	return solve(_input, 4)

def part2(_input: str) -> int:
	return solve(_input, 14)

def solve(_input: str, max_size: int) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		stream = file.read()
		window_size = 1
		window_start = None
		for window_end, char in enumerate(stream[1:]):
			new_size = 0
			for previousChar in stream[window_end:window_start:-1]:
				if char != previousChar: new_size += 1
				else:
					window_start = window_end - new_size
					break
			window_size = new_size + 1
			if window_size == max_size:
				return window_end + 2

_input = "./2022/Day6/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
