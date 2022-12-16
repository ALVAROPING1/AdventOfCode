from re import findall

def part1(_input: str) -> int:
	y_coordinate = 2_000_000
	with open(_input, "r", encoding="UTF-8") as file:
		sensors = tuple(tuple(map(int, findall(r"-?\d+", sensor))) for sensor in file.readlines())
	ranges = tuple(abs(sensor[0] - sensor[2]) + abs(sensor[1] - sensor[3]) for sensor in sensors)
	minX, maxX = min(sensor[0] - _range for sensor, _range in zip(sensors, ranges)), max(sensor[0] + _range for sensor, _range in zip(sensors, ranges))
	scanned_positions = [False for _ in range(maxX - minX)]
	for sensor, _range in zip(sensors, ranges):
		range_x = _range - abs(sensor[1] - y_coordinate)
		if range_x >= 0:
			scanned_range = (sensor[0] - range_x - minX, sensor[0] + range_x + 1 - minX)
			scanned_positions[scanned_range[0]:scanned_range[1]] = [True for _ in range(scanned_range[0], scanned_range[1])]
	for sensor in sensors:
		if sensor[3] == y_coordinate:
			scanned_positions[sensor[2] - maxX] = False
	return sum(scanned_positions)

def part2(_input: str) -> int:
	max_signal_position = 4_000_000
	with open(_input, "r", encoding="UTF-8") as file:
		sensors = tuple(tuple(map(int, findall(r"-?\d+", sensor))) for sensor in file.readlines())
	ranges = tuple(abs(sensor[0] - sensor[2]) + abs(sensor[1] - sensor[3]) for sensor in sensors)
	minX, maxX = min(sensor[0] - _range for sensor, _range in zip(sensors, ranges)), max(sensor[0] + _range for sensor, _range in zip(sensors, ranges))
	scanned_positions = [[False for _ in range(maxX - minX)] for _ in range(max_signal_position + 1)]
	for sensor, _range in zip(sensors, ranges):
		print("next")
		for y_coordinate in range(max_signal_position + 1):
			range_x = _range - abs(sensor[1] - y_coordinate)
			if range_x >= 0:
				scanned_range = (sensor[0] - range_x - minX, sensor[0] + range_x + 1 - minX)
				scanned_positions[y_coordinate][scanned_range[0]:scanned_range[1]] = [True for _ in range(scanned_range[0], scanned_range[1])]
	for sensor in sensors:
		if sensor[3] <= max_signal_position:
			scanned_positions[max_signal_position][sensor[2] - maxX] = False
	position = next(find((line[-minX:max_signal_position-minX+1] for line in scanned_positions), False))
	return position[0] + position[1] * max_signal_position

def find(matrix: tuple[tuple[bool]], value: bool) -> tuple[int, int]:
    for i, line in enumerate(matrix):
        try:
            j = line.index(value)
        except ValueError:
            continue
        yield i, j

_input = "./2022/Day15/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
