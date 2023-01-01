from dataclasses import dataclass
from re import search, finditer
from itertools import permutations, combinations

def part1(_input: str) -> int:
	graph, start_id = Graph.from_file(_input)
	return graph.get_max_pressure(start_id, 30, 12, 14)[0]

def part2(_input: str) -> int:
	graph, start_id = Graph.from_file(_input)
	return get_max_2_partition(graph.get_max_pressure(start_id, 26, 6, 7)[1], start_id)

@dataclass
class Graph:
	edges: list[list[int]]
	vertex_values: list[int]

	@classmethod
	def from_file(cls, _input: str) -> tuple["Graph", int]:
		graph = cls(None, [])
		str2id = {}
		adj_vertices_str = []
		with open(_input, "r", encoding="UTF-8") as file:
			for _id, valve in enumerate(file.readlines()):
				str2id[valve[6:8]] = _id
				graph.vertex_values.append(int(search(r"\d+", valve[23:25]).group()))
				adj_vertices_str.append(valve[49:])
		adj_vertices = tuple(
			tuple(str2id[v.group()] for v in finditer(r"[A-Z][A-Z]", _valve))
			for _valve in adj_vertices_str
		)
		graph.calculate_distance_matrix(adj_vertices)
		return cls.remove_stuck_valves(graph, str2id["AA"])

	def calculate_distance_matrix(self, adj_vertices: tuple[tuple[int]]):
		self.edges = [[1 << 30 for _ in range(len(adj_vertices))] for _ in range(len(adj_vertices))]
		for start, adj in enumerate(adj_vertices):
			self.edges[start][start] = 0
			for end in adj:
				self.edges[start][end] = 1
				self.edges[end][start] = 1
		for k, i, j in permutations(range(len(adj_vertices)), r=3):
			self.edges[i][j] = min(
				self.edges[i][j],
				self.edges[i][k] + self.edges[k][j]
			)

	@classmethod
	def remove_stuck_valves(cls, graph: "Graph", start_id: int) -> tuple["Graph", int]:
		new_graph = cls([], [])
		new_start_id = start_id
		for start, adj_vertices in enumerate(graph.edges):
			if graph.vertex_values[start] > 0 or start == start_id:
				current = []
				for end, distance in enumerate(adj_vertices):
					if graph.vertex_values[end] > 0 or end == start_id:
						current.append(distance)
				new_graph.edges.append(current)
				new_graph.vertex_values.append(graph.vertex_values[start])
			elif start < start_id:
				new_start_id -= 1
		return new_graph, new_start_id

	def get_max_pressure(
		self,
		start_vertex: int,
		start_time: int,
		min_flow_1: int,
		min_time_1: int,
	) -> tuple[int, dict[int, int]]:
		max_pressure = 0
		max_pressure_dict = {}
		visited = 1 << start_vertex
		stack = [(start_time, visited, start_vertex, max_pressure)]
		while stack:
			time, visited, current_vertex, current_pressure = stack.pop()
			max_pressure = max(max_pressure, current_pressure)
			if visited not in max_pressure_dict or max_pressure_dict[visited] < current_pressure:
				max_pressure_dict[visited] = current_pressure
			for adjacent, flow in enumerate(self.vertex_values):
				if (
					visited & 1 << adjacent == 0
					and time > self.edges[current_vertex][adjacent]
					and (flow >= min_flow_1 or time < min_time_1)
				):
					new_time = time - self.edges[current_vertex][adjacent] - 1
					stack.append((
						new_time,
						visited | 1 << adjacent,
						adjacent,
						current_pressure + flow * new_time
					))
		return max_pressure, max_pressure_dict

def get_max_2_partition(_dict: dict[int, int], start_vertex: int) -> int:
	max_value = 0
	for key1, key2 in combinations(_dict, r=2):
		if key1 & key2 == 1 << start_vertex:
			max_value = max(max_value, _dict[key1] + _dict[key2])
	return max_value

_input = "./2022/Day16/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
