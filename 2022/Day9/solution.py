def part1(_input: str) -> int:
	letter2axis = {"L": 0, "R": 0, "U": 1, "D": 1}
	letter2direction = {"L": -1, "R": 1, "U": 1, "D": -1}
	with open(_input, "r", encoding="UTF-8") as file:
		moves = tuple((letter2axis[x[0]], letter2direction[x[0]], int(x [2:])) for x in file.readlines())
	maxX, _, minX, minY = calculate_grid_size(moves)
	current_head_position = [-minX, -minY]
	tail = RopeSegment(*current_head_position)
	line_length = maxX - minX
	visited_positions = matrix_positions2bit_field((current_head_position,), line_length)
	for move in moves:
		current_head_position[move[0]] += move[1] * move[2]
		new_positions = tail.move_towards_point(*current_head_position)
		new_bit_field = matrix_positions2bit_field(new_positions, line_length)
		visited_positions = visited_positions | new_bit_field
	return visited_positions.bit_count()

def part2(_input: str) -> int:
	letter2axis = {"L": 0, "R": 0, "U": 1, "D": 1}
	letter2direction = {"L": -1, "R": 1, "U": 1, "D": -1}
	with open(_input, "r", encoding="UTF-8") as file:
		moves = tuple((letter2axis[x[0]], letter2direction[x[0]], int(x [2:])) for x in file.readlines())
	maxX, _, minX, minY = calculate_grid_size(moves)
	current_head_position = [-minX, -minY]
	mid_sections = tuple(RopeSegment(*current_head_position) for _ in range(8))
	tail = RopeSegment(*current_head_position)
	line_length = maxX - minX
	visited_positions = matrix_positions2bit_field((current_head_position,), line_length)
	for move in moves:
		for _ in range(move[2]):
			current_head_position[move[0]] += move[1]
			previous_rope_position = current_head_position
			for rope in mid_sections:
				new_mid_points = rope.move_towards_point(*previous_rope_position)
				if len(new_mid_points) == 0:
					break
				previous_rope_position = (rope.positionX, rope.positionY)
			if len(new_mid_points) != 0:
				new_positions = tail.move_towards_point(mid_sections[-1].positionX, mid_sections[-1].positionY)
				new_bit_field = matrix_positions2bit_field(new_positions, line_length)
				visited_positions = visited_positions | new_bit_field
	return visited_positions.bit_count()

class RopeSegment():
	def __init__(self, x: int, y: int) -> None:
		self.positionX = x
		self.positionY = y
	
	def move_towards_point(self, x: int, y: int) -> list[tuple]:
		deltaX, deltaY = x-self.positionX, y-self.positionY
		if abs(deltaX) <= 1 and abs(deltaY) <= 1:
			return []
		out = []
		if deltaX * deltaY != 0:
			self.positionX += 1 if deltaX > 0 else -1
			self.positionY += 1 if deltaY > 0 else -1
			out.append((self.positionX, self.positionY))
		out.extend(self._move_towards_point_straight(x, y))
		self.positionX, self.positionY = out[-1]
		return out
	
	def _move_towards_point_straight(self, x: int, y: int) -> list[tuple]:
		deltaX = x - self.positionX
		if deltaX == 0:
			sign = 1 if y - self.positionY > 0 else -1
			return ((self.positionX, position) for position in range(self.positionY + sign, y, sign))
		sign = 1 if deltaX > 0 else -1
		return ((position, self.positionY) for position in range(self.positionX + sign, x, sign))
	
def calculate_grid_size(moves: tuple[tuple[int, int, int]]) -> tuple[int, int, int, int]:
	max_position = [0, 0]
	min_position = [0, 0]
	current_position = [0, 0]
	for move in moves:
		axis = move[0]
		current_position[axis] += move[1] * move[2]
		max_position[axis] = max(max_position[axis], current_position[axis])
		min_position[axis] = min(min_position[axis], current_position[axis])
	return *max_position, *min_position

def matrix_positions2bit_field(position: tuple[tuple[int, int]], line_length: int) -> int:
	return sum(2 ** (i + line_length * j) for i, j, in position)

_input = "./2022/Day9/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
