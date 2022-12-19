from typing import Type

def part1(_input: str) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		movements = file.read()[:-1]
	chamber = Chamber(7, 5300, movements)
	rock_types = (HorizontalLine, Plus, J, VerticalLine, O)
	for i in range(2022):
		chamber.spawn_rock(rock_types[i % 5])
	return chamber.max_height

def part2(_input: str, rocks: int) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		movements = file.read()[:-1]
	chamber = Chamber(7, 5300, movements)
	rock_types = (HorizontalLine, Plus, J, VerticalLine, O)
	heights = {}
	rocks_per_cycle = None
	for i in range(rocks):
		rock_type = rock_types[i % 5]
		if rocks_per_cycle is None:
			heights_key = (rock_type, chamber.movements_index)
			if heights_key not in heights:
				heights[heights_key] = (chamber.max_height, i)
			else:
				heights_value = heights[heights_key]
				if chamber.map[heights_value[0] - 25] == chamber.map[chamber.max_height - 25]:
					height_per_cycle = chamber.max_height - heights_value[0]
					rocks_per_cycle = i - heights_value[1]
					if (rocks - i) % rocks_per_cycle == 0:
						break
		elif (rocks - i) % rocks_per_cycle == 0:
			break
		chamber.spawn_rock(rock_type)
	return chamber.max_height + (rocks - i) * height_per_cycle // rocks_per_cycle

class Rock:
	@staticmethod
	def check_left(_map: tuple[tuple[bool]], x: int, y: int) -> bool:
		raise NotImplementedError

	@staticmethod
	def check_right(_map: tuple[tuple[bool]], x: int, y: int) -> bool:
		raise NotImplementedError

	@staticmethod
	def check_down(_map: tuple[tuple[bool]], x: int, y: int) -> bool:
		raise NotImplementedError

	@staticmethod
	def get_points(x: int, y: int) -> tuple[tuple[int, int]]:
		raise NotImplementedError

	@staticmethod
	def get_highest_point(y: int) -> int:
		raise NotImplementedError

class O(Rock):
	@staticmethod
	def check_left(_map: tuple[tuple[bool]], x: int, y: int) -> bool:
		return _map[y][x-1] and _map[y+1][x-1]

	@staticmethod
	def check_right(_map: tuple[tuple[bool]], x: int, y: int) -> bool:
		return _map[y][x+2] and _map[y+1][x+2]

	@staticmethod
	def check_down(_map: tuple[tuple[bool]], x: int, y: int) -> bool:
		return _map[y-1][x] and _map[y-1][x+1]

	@staticmethod
	def get_points(x: int, y: int) -> tuple[tuple[int, int]]:
		return ((x, y), (x+1, y), (x, y+1), (x+1, y+1))

	@staticmethod
	def get_highest_point(y: int) -> int:
		return y+1

class VerticalLine(Rock):
	@staticmethod
	def check_left(_map: tuple[tuple[bool]], x: int, y: int) -> bool:
		return _map[y][x-1] and _map[y+1][x-1] and _map[y+2][x-1] and _map[y+3][x-1]

	@staticmethod
	def check_right(_map: tuple[tuple[bool]], x: int, y: int) -> bool:
		return _map[y][x+1] and _map[y+1][x+1] and _map[y+2][x+1] and _map[y+3][x+1]

	@staticmethod
	def check_down(_map: tuple[tuple[bool]], x: int, y: int) -> bool:
		return _map[y-1][x]

	@staticmethod
	def get_points(x: int, y: int) -> tuple[tuple[int, int]]:
		return ((x, y), (x, y+1), (x, y+2), (x, y+3))

	@staticmethod
	def get_highest_point(y: int) -> int:
		return y+3

class J(Rock):
	@staticmethod
	def check_left(_map: tuple[tuple[bool]], x: int, y: int) -> bool:
		return _map[y][x-1] and _map[y+2][x+1]

	@staticmethod
	def check_right(_map: tuple[tuple[bool]], x: int, y: int) -> bool:
		return _map[y][x+3] and _map[y+1][x+3] and _map[y+2][x+3]

	@staticmethod
	def check_down(_map: tuple[tuple[bool]], x: int, y: int) -> bool:
		return _map[y-1][x] and _map[y-1][x+1] and _map[y-1][x+2]

	@staticmethod
	def get_points(x: int, y: int) -> tuple[tuple[int, int]]:
		return ((x, y), (x+1, y), (x+2, y), (x+2, y+1), (x+2, y+2))

	@staticmethod
	def get_highest_point(y: int) -> int:
		return y+2

class Plus(Rock):
	@staticmethod
	def check_left(_map: tuple[tuple[bool]], x: int, y: int) -> bool:
		return _map[y][x] and _map[y+1][x-1]

	@staticmethod
	def check_right(_map: tuple[tuple[bool]], x: int, y: int) -> bool:
		return _map[y][x+2] and _map[y+1][x+3]

	@staticmethod
	def check_down(_map: tuple[tuple[bool]], x: int, y: int) -> bool:
		return _map[y][x] and _map[y-1][x+1] and _map[y][x+2]

	@staticmethod
	def get_points(x: int, y: int) -> tuple[tuple[int, int]]:
		return ((x+1, y), (x, y+1), (x+2, y+1), (x+1, y+2))

	@staticmethod
	def get_highest_point(y: int) -> int:
		return y+2

class HorizontalLine(Rock):
	@staticmethod
	def check_left(_map: tuple[tuple[bool]], x: int, y: int) -> bool:
		return _map[y][x-1]

	@staticmethod
	def check_right(_map: tuple[tuple[bool]], x: int, y: int) -> bool:
		return _map[y][x+4]

	@staticmethod
	def check_down(_map: tuple[tuple[bool]], x: int, y: int) -> bool:
		return _map[y-1][x] and _map[y-1][x+1] and _map[y-1][x+2] and _map[y-1][x+3]

	@staticmethod
	def get_points(x: int, y: int) -> tuple[tuple[int, int]]:
		return ((x, y), (x+1, y), (x+2, y), (x+3, y))

	@staticmethod
	def get_highest_point(y: int) -> int:
		return y

class Chamber:
	def __init__(self, width: int, height: int, movements: str) -> None:
		self.map = [[False] + [True for _ in range(width)] + [False] for _ in range(height)]
		self.map[0] = [False for _ in range(9)]
		self.max_height = 0
		self.movements_index = 0
		self.movements = movements

	def spawn_rock(self, rock_type: Type[Rock]) -> None:
		position = [3, self.max_height + 4]
		while True:
			if self.movements[self.movements_index] == ">":
				if rock_type.check_right(self.map, *position):
					position[0] += 1
			else:
				if rock_type.check_left(self.map, *position):
					position[0] -= 1
			self.movements_index = (self.movements_index + 1) % len(self.movements)
			if rock_type.check_down(self.map, *position):
				position[1] -= 1
			else:
				break
		for x,y in rock_type.get_points(*position):
			self.map[y][x] = False
		self.max_height = max(self.max_height, rock_type.get_highest_point(position[1]))

_input = "./2022/Day17/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input, 1000000000000))
