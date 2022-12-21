def part1(_input: str) -> int:
	_file = parse_file(_input, 1)
	mix_file_once(_file)
	return get_coordinate_from_file(_file)

def part2(_input: str) -> int:
	_file = parse_file(_input, 811589153)
	mix_file_repeated(_file, 10)
	return get_coordinate_from_file(_file)

def parse_file(_input: str, decryption_key: int) -> list[tuple[int, int]]:
	with open(_input, "r", encoding="UTF-8") as file:
		_file = list((index, decryption_key * number) for index, number in enumerate(map(
			int, file.read().split("\n")[:-1])
		))
	return _file

def mix_file_once(_file: list[tuple[int, int]]) -> None:
	seen_numbers = set()
	current_index = 0
	while current_index < len(_file):
		if _file[current_index][0] in seen_numbers or _file[current_index][1] == 0:
			current_index += 1
			continue
		element = _file.pop(current_index)
		new_index = (current_index + element[1]) % len(_file)
		_file.insert(new_index, element)
		seen_numbers.add(element[0])
		if new_index <= current_index:
			current_index += 1

def mix_file_repeated(_file: list[tuple[int, int]], amount: int) -> None:
	order = _file.copy()
	mix_file_once(_file)
	for _ in range(amount-1):
		for element in order:
			if element[1] == 0:
				continue
			current_index = _file.index(element)
			_file.pop(current_index)
			new_index = (current_index + element[1]) % len(_file)
			_file.insert(new_index, element)

def get_coordinate_from_file(_file: list[tuple[int, int]]) -> int:
	for index, value in enumerate(element[1] for element in _file):
		if value == 0:
			zero_index = index
			break
	return sum(_file[(zero_index + i * 1000) % len(_file)][1] for i in range(1,4))

_input = "./2022/Day20/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
