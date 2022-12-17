from re import findall

def part1(_input: str) -> int:
	y_coordinate = 2_000_000
	sensors, ranges = parse_input(_input)
	scanned_positions = check_y_coordinate(sensors, ranges, y_coordinate)
	for sensor in sensors:
		if sensor[3] == y_coordinate:
			for index, _range in enumerate(scanned_positions.ranges):
				if sensor[2] > _range[1]:
					continue
				if sensor[2] < _range[0]:
					break
				if sensor[2] == _range[0]:
					_range[0] += 1
				elif sensor[2] == _range[1]:
					_range[1] -= 1
				else:
					scanned_positions.ranges.insert(index + 1, [sensor[2] + 1, _range[1]])
					_range[1] = sensor[2] - 1
				break
	return sum(_range[1] - _range[0] + 1 for _range in scanned_positions.ranges)

def part2(_input: str) -> int:
	max_signal_position = 4_000_000
	sensors, ranges = parse_input(_input)
	for y_coordinate in range(max_signal_position):
		scanned_positions = check_y_coordinate(sensors, ranges, y_coordinate)
		if not scanned_positions.check_full_range(0, max_signal_position):
			return scanned_positions.get_missing_point() * 4_000_000 + y_coordinate

class ScannedPositions:
	def __init__(self) -> None:
		self.ranges = []

	def add_range(self, start: int, end: int) -> None:
		for index, _range in enumerate(self.ranges):
			if start > _range[1] + 1:
				continue
			if end < _range[0] - 1:
				self.ranges.insert(index, [start, end])
				return
			if start >= _range[0] and end <= _range[1]:
				return
			if start < _range[0]:
				_range[0] = start
			if end > _range[1]:
				_range[1] = end
				self._merge_right(index)
			return
		self.ranges.append([start, end])

	def _merge_right(self, index: int) -> None:
		if index == len(self.ranges) - 1 or self.ranges[index][1] < self.ranges[index + 1][0] - 1:
			return
		self.ranges[index][1] = max(self.ranges[index][1], self.ranges[index + 1][1])
		self.ranges.pop(index + 1)
		self._merge_right(index)

	def check_full_range(self, start: int, end: int) -> bool:
		return len(self.ranges) == 1 and self.ranges[0][0] <= start and self.ranges[0][1] >= end

	def get_missing_point(self) -> int:
		if self.ranges[0][0] == 0:
			return self.ranges[0][1] + 1
		return self.ranges[0][0] - 1

def parse_input(_input: str) -> tuple[tuple[tuple[int]], tuple[int]]:
	with open(_input, "r", encoding="UTF-8") as file:
		sensors = tuple(tuple(map(int, findall(r"-?\d+", sensor))) for sensor in file.readlines())
	ranges = tuple(abs(sensor[0] - sensor[2]) + abs(sensor[1] - sensor[3]) for sensor in sensors)
	return sensors, ranges

def check_y_coordinate(sensors: tuple[tuple[int]], ranges: tuple[int], y_coordinate: int) -> ScannedPositions:
	scanned_positions = ScannedPositions()
	for sensor, _range in zip(sensors, ranges):
		range_x = _range - abs(sensor[1] - y_coordinate)
		if range_x >= 0:
			scanned_range = (sensor[0] - range_x, sensor[0] + range_x)
			scanned_positions.add_range(*scanned_range)
	return scanned_positions

_input = "./2022/Day15/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
