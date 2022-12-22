from dataclasses import dataclass
from re import search, finditer
from itertools import permutations

def part1(_input: str) -> int:
	graph = Graph.from_file(_input)
	return graph.DFT({v: v == "AA" for v in graph.vertices}, "AA", 30, 1, 12, 14)

def part2(_input: str) -> int:
	graph = Graph.from_file(_input)
	return graph.DFT({v: v == "AA" for v in graph.vertices}, "AA", 26, 2, 6, 7, 12, 14)

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

	def DFT(
		self,
		visited: dict[str, bool],
		start_vertex: str,
		start_time: int,
		explorers: int,
		min_flow_1: int,
		min_time_1: int,
		min_flow_2: int = 0,
		min_time_2: int = 0
	) -> int:
		max_pressure = 0
		stack = [(start_time, visited, start_vertex, max_pressure)]
		while stack:
			time, visited, current_vertex, current_pressure = stack.pop()
			visited[current_vertex] = True
			finished = True
			for adjacent in self.edges[current_vertex]:
				if (
					not visited[adjacent]
					and time > self.edges[current_vertex][adjacent]
					and (self.vertex_values[adjacent] >= min_flow_1 or time < min_time_1)
				):
					new_time = time - self.edges[current_vertex][adjacent] - 1
					stack.append((
						new_time,
						visited.copy(),
						adjacent,
						current_pressure + (self.vertex_values[adjacent]) * new_time
					))
					finished = False
			if explorers > 1 and finished:
				max_pressure = max(max_pressure, current_pressure + self.DFT(
						visited,
						start_vertex,
						start_time,
						explorers - 1,
						min_flow_2,
						min_time_2
					)
				)
			else:
				max_pressure = max(max_pressure, current_pressure)
		return max_pressure

_input = "./2022/Day16/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
