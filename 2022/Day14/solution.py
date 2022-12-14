from itertools import chain
from time import sleep

def part1(_input: str) -> int:
	sand_spawn = (500, 0)
	scan, minX, minY = parse_input(_input, sand_spawn, 0, 0, 0)
	count = 0
	sand_spawn_reduced = (sand_spawn[0] - minX, sand_spawn[1] - minY)
	while spawn_sand(*sand_spawn_reduced, scan):
		count += 1
	return count

def part2(_input: str) -> int:
	sand_spawn = (500, 0)
	scan, minX, minY = parse_input(_input, sand_spawn, 90, 155, 2)
	for column in scan:
		column[len(scan[0]) - 1] = False
	count = 0
	sand_spawn_reduced = (sand_spawn[0] - minX, sand_spawn[1] - minY)
	while scan[sand_spawn_reduced[0]][sand_spawn_reduced[1]]:
		spawn_sand(*sand_spawn_reduced, scan)
		count += 1
	#for column in scan:
	#	print("".join("." if v else "#" for v in column))
	#print("================================================================================================================")
	return count

def parse_input(_input: str, sand_spawn: tuple[int, int], extendXNegative: int, extendXPositive: int, extendY: int) -> tuple[list[list[bool]], int, int]:
	with open(_input, "r", encoding="UTF-8") as file:
		paths = tuple((tuple((int(point[:3]), int(point[4:])) for point in line.split(" -> "))) for line in file.readlines())
	points = tuple(zip(*chain.from_iterable(paths), sand_spawn))
	maxX, minX, maxY, minY = max(points[0]) + extendXPositive, min(points[0]) - extendXNegative, max(points[1]) + extendY, min(points[1])
	scan = [[True for _ in range(maxY - minY + 1)] for _ in range(maxX - minX + 1)]
	for path in paths:
		previous = path[0]
		scan[previous[0] - minX][previous[1] - minY] = False
		for end_point in path[1:]:
			direction = previous[0] == end_point[0]
			sign = 1 if end_point[direction] - previous[direction] > 0 else -1
			if direction:
				for point in range(previous[1] - minY, end_point[1] + sign - minY, sign):
					scan[end_point[0] - minX][point] = False
			else:
				for point in range(previous[0] - minX, end_point[0] + sign - minX, sign):
					scan[point][end_point[1] - minY] = False
			previous = end_point
	return scan, minX, minY


def spawn_sand(x: int, y: int, scan: list[list[bool]]) -> bool:
	try:
		while True:
			y += 1
			if scan[x][y]:
				pass
			elif scan[x-1][y]:
				x -= 1
			elif scan[x+1][y]:
				x += 1
			else:
				scan[x][y-1] = False
				return True
	except IndexError:
		return False

_input = "./2022/Day14/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
