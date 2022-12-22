from dataclasses import dataclass
from re import search, finditer
from itertools import permutations, combinations

def part1(_input: str) -> int:
	graph = Graph.from_file(_input)
	return graph.get_max_pressure("AA", 30, 12, 14)[0]

def part2(_input: str) -> int:
	graph = Graph.from_file(_input)
	return get_max_2_partition(graph.get_max_pressure("AA", 26, 6, 7)[1])

@dataclass
class Graph:
	edges: dict[str, dict[str, int]]
	vertex_values: dict[str, int]

	@property
	def vertices(self) -> tuple[str]:
		return self.vertex_values.keys()

	def get_edge_weight(self, start: str, end: str) -> int:
		try:
			return self.edges[start][end]
		except KeyError:
			return 100000

	@classmethod
	def from_file(cls, _input: str) -> "Graph":
		graph = cls({}, {})
		with open(_input, "r", encoding="UTF-8") as file:
			for valve in file.readlines():
				current = valve[6:8]
				graph.vertex_values[current] = int(search(r"\d+", valve[23:25]).group())
				graph.edges[current] = {}
				graph.edges[current][current] = 0
				for end in finditer(r"[A-Z][A-Z]", valve[49:]):
					graph.edges[current][end.group()] = 1
		graph.convert_to_distance_matrix()
		graph.remove_stuck_valves()
		return graph

	def convert_to_distance_matrix(self):
		for k, i, j in permutations(self.vertices, r=3):
			self.edges[i][j] = min(
				self.get_edge_weight(i, j),
				self.get_edge_weight(i, k) + self.get_edge_weight(k, j)
			)

	def remove_stuck_valves(self):
		for vertex, value in self.vertex_values.copy().items():
			if value != 0:
				continue
			for end in self.edges[vertex].copy().keys():
				del self.edges[end][vertex]
			if vertex != "AA":
				del self.edges[vertex]
				del self.vertex_values[vertex]
				try:
					del self.edges["AA"][vertex]
				except KeyError:
					continue

	def get_max_pressure(
		self,
		start_vertex: str,
		start_time: int,
		min_flow_1: int,
		min_time_1: int,
	) -> tuple[int, dict[frozenset[str], int]]:
		max_pressure = 0
		max_pressure_dict = {}
		visited = frozenset()
		stack = [(start_time, visited, start_vertex, max_pressure)]
		while stack:
			time, visited, current_vertex, current_pressure = stack.pop()
			max_pressure = max(max_pressure, current_pressure)
			if visited not in max_pressure_dict or max_pressure_dict[visited] < current_pressure:
				max_pressure_dict[visited] = current_pressure
			for adjacent in self.edges[current_vertex]:
				if (
					adjacent not in visited
					and time > self.edges[current_vertex][adjacent]
					and (self.vertex_values[adjacent] >= min_flow_1 or time < min_time_1)
				):
					new_time = time - self.edges[current_vertex][adjacent] - 1
					stack.append((
						new_time,
						visited | frozenset({adjacent}),
						adjacent,
						current_pressure + (self.vertex_values[adjacent]) * new_time
					))
		return max_pressure, max_pressure_dict

def get_max_2_partition(_dict: dict[frozenset[str], int]) -> int:
	max_value = 0
	for key1, key2 in combinations(_dict, r=2):
		if key1 & key2 == set():
			max_value = max(max_value, _dict[key1] + _dict[key2])
	return max_value

_input = "./2022/Day16/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
