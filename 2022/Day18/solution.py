from typing import Iterable
from collections import deque

def part1(_input: str) -> int:
	map_size = 22
	with open(_input, "r", encoding="UTF-8") as file:
		cubes = tuple(tuple(map(int, cube[:-1].split(","))) for cube in file.readlines())
	map_lava = process_cubes(cubes, map_size)
	return calculate_surface_area(map_lava, cubes)

def part2(_input: str) -> int:
	map_size = 22
	with open(_input, "r", encoding="UTF-8") as file:
		map_lava = process_cubes(
			(map(int, cube[:-1].split(",")) for cube in file.readlines()),
			map_size
		)
	return calculate_external_surface_area(map_lava, map_size)

def process_cubes(cubes: Iterable, map_size: int) -> tuple[tuple[tuple[bool]]]:
	_map = [[[False for _ in range(map_size)] for _ in range(map_size)] for _ in range(map_size)]
	for x,y,z in cubes:
		_map[x+1][y+1][z+1] = True
	return _map

def calculate_surface_area(_map: list[list[list[bool]]], cubes: Iterable) -> int:
	surface_area = 0
	for x,y,z in cubes:
		surface_area += 6 - sum(
			_map[_x+1][_y+1][_z+1] for _x, _y, _z in get_adjacent_points(x, y, z)
		)
	return surface_area

def calculate_external_surface_area(
	map_lava: tuple[tuple[tuple[bool]]],
	map_size: int
) -> int:
	map_steam = [[[False for _ in range(map_size)] for _ in range(map_size)] for _ in range(map_size)]
	queue = deque([(0,0,0)])
	surface_area = 0
	while queue:
		x, y, z = queue.popleft()
		for _x, _y, _z in get_adjacent_points(x, y, z):
			if (
				0 <= _x < map_size
				and 0 <= _y < map_size
				and 0 <= _z < map_size
				and not map_steam[_x][_y][_z]
			):
				if not map_lava[_x][_y][_z]:
					queue.append((_x, _y, _z))
					map_steam[_x][_y][_z] = True
				else:
					surface_area += 1
	return surface_area

def get_adjacent_points(x: int, y: int, z: int):
	return (
		(x+1, y, z),
		(x-1, y, z),
		(x, y+1, z),
		(x, y-1, z),
		(x, y, z+1),
		(x, y, z-1)
	)

_input = "./2022/Day18/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
