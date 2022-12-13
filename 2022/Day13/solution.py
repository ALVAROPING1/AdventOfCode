from ast import literal_eval

def part1(_input: str) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		return sum((index+1) * compare_lists(*map(literal_eval, pair.split("\n"))) for index, pair in enumerate(file.read()[:-1].split("\n\n")))

def part2(_input: str) -> int:
	index1, index2 = 1, 1
	separator1, separator2 = [[2]], [[6]]
	with open(_input, "r", encoding="UTF-8") as file:
		for packet in file.readlines():
			if packet == "\n":
				continue
			parsed_packet = literal_eval(packet[:-1])
			if compare_lists(parsed_packet, separator1):
				index1 += 1
			else:
				index2 += compare_lists(parsed_packet, separator2)
	return index1 * (index1 + index2)

def compare_lists(first: list[int], second: list[int]) -> bool:
	first_index = 0
	second_index = 0
	while first_index < len(first) and second_index < len(second):
		current1 = first[first_index]
		current2 = second[second_index]
		if isinstance(current1, int) and isinstance(current2, int):
			if current1 != current2:
				return current1 < current2
		elif isinstance(current1, list) and isinstance(current2, list):
			comparison = compare_lists(current1, current2)
			if comparison is not None:
				return comparison
		elif isinstance(current1, list):
			comparison = compare_lists(current1, [current2])
			if comparison is not None:
				return comparison
		else:
			comparison = compare_lists([current1], current2)
			if comparison is not None:
				return comparison
		first_index += 1
		second_index += 1
	return None if len(first) == len(second) else len(first) < len(second)

_input = "./2022/Day13/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
