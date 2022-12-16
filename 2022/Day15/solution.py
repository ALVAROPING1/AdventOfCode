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
	for y_coordinate in range(max_signal_position):
		out = check_y_coordinate(sensors, ranges, y_coordinate, max_signal_position)
		if out != -1:
			return out

def check_y_coordinate(sensors: tuple[tuple[int]], ranges: tuple[int], y_coordinate: int, max_signal_position: int):
	scanned_positions = ScannedPositions()
	for sensor, _range in zip(sensors, ranges):
		range_x = _range - abs(sensor[1] - y_coordinate)
		if range_x >= 0:
			scanned_range = (max(sensor[0] - range_x, 0), min(sensor[0] + range_x, max_signal_position))
			scanned_positions.add_range(*scanned_range)
			if scanned_positions.check_full_range(0, max_signal_position): return -1
	scanned_positions._merge_ranges()
	if not scanned_positions.check_full_range(0, max_signal_position):
		return scanned_positions.get_missing_point() * 4_000_000 + y_coordinate

class ScannedPositions:
	def __init__(self) -> None:
		self.ranges = []
	
	def add_range(self, start: int, end: int) -> None:
		self._merge_ranges()
		self._add_range(start, end)

	def _add_range(self, start: int, end: int) -> None:
		for _range in self.ranges:
			if start >= _range[0] and end <= _range[1]:
				return
			if start > _range[1] + 1 or end < _range[0] - 1:
				continue
			if start < _range[0]:
				_range[0] = start
			if end > _range[1]:
				_range[1] = end
			return
		self.ranges.append([start, end])
	
	def _merge_ranges(self) -> None:
		old_ranges = self.ranges
		self.ranges = []
		for _range in old_ranges:
			self._add_range(*_range)
		if len(old_ranges) != len(self.ranges):
			self._merge_ranges()

	def check_full_range(self, start: int, end: int) -> bool:
		return len(self.ranges) == 1 and self.ranges[0][0] <= start and self.ranges[0][1] >= end
	
	def get_missing_point(self) -> int:
		if self.ranges[0][0] == 0:
			return self.ranges[0][1] + 1
		return self.ranges[0][0] - 1

_input = "./2022/Day15/input.txt"

print("Part 1:", part1(_input))
# 3187704 < sol
print("Part 2:", part2(_input))
