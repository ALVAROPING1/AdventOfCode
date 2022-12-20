from typing import Iterable
from collections import deque
from copy import deepcopy

def part1(_input: str) -> int:
	_map = [[[False for _ in range(22)] for _ in range(22)] for _ in range(22)]
	with open(_input, "r", encoding="UTF-8") as file:
		return calculate_surface_area(
			_map,
			(map(int, cube[:-1].split(",")) for cube in file.readlines())
		)

def part2(_input: str) -> int:
	map_lava = [[[False for _ in range(22)] for _ in range(22)] for _ in range(22)]
	map_steam = deepcopy(map_lava)
	map_air = deepcopy(map_lava)
	with open(_input, "r", encoding="UTF-8") as file:
		maximum_area = calculate_surface_area(
			map_lava,
			(map(int, cube[:-1].split(",")) for cube in file.readlines())
		)
	queue = deque()
	queue.append((0,0,0))
	total_steam = 0
	while queue:
		x, y, z = queue.popleft()
		for _x, _y, _z in (
				(x+1, y, z),
				(x-1, y, z),
				(x, y+1, z),
				(x, y-1, z),
				(x, y, z+1),
				(x, y, z-1)
		):
			if (
				0 <= _x < 22
				and 0 <= _y < 22
				and 0 <= _z < 22
				and not map_steam[_x][_y][_z]
				and not map_lava[_x][_y][_z]
			):
				queue.append((_x, _y, _z))
				map_steam[_x][_y][_z] = True
				total_steam += 1
	return

def calculate_surface_area(_map: list[list[list[int]]], cubes: Iterable) -> int:
	surface_area = 0
	for x,y,z in cubes:
		surface_area += 6 - 2 * sum(
			_map[_x+1][_y+1][_z+1] for _x, _y, _z in (
				(x+1, y, z),
				(x-1, y, z),
				(x, y+1, z),
				(x, y-1, z),
				(x, y, z+1),
				(x, y, z-1)
			)
		)
		_map[x+1][y+1][z+1] = True
	return surface_area

_input = "./2022/Day18/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
