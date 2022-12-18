from math import inf
from re import search, finditer
from itertools import product

def part1(_input: str) -> int:
	graph, vertex_values = parse_input(_input)
	return DFT(graph, {v: False for v in vertex_values}, "AA", vertex_values, 30)

def part2(_input: str) -> int:
	graph, vertex_values = parse_input(_input)
	return DFT_pairs(graph, {v: False for v in vertex_values}, "AA", vertex_values, 26)


def parse_input(_input: str) -> tuple[dict[str, dict[str, int]], dict[str, int]]:
	graph = {}
	vertex_values = {}
	with open(_input, "r", encoding="UTF-8") as file:
		for valve in file.readlines():
			current = valve[6:8]
			vertex_values[current] = int(search(r"\d+", valve[23:25]).group())
			graph[current] = {}
			graph[current][current] = 0
			for end in finditer(r"[A-Z][A-Z]", valve[49:]):
				graph[current][end.group()] = 1
	calculate_distance_matrix(graph, vertex_values.keys())
	remove_stuck_valves(graph, vertex_values)
	return graph, vertex_values


def calculate_distance_matrix(graph: dict[str, dict[str, int]], vertices: tuple[str]):
	for k, i, j in product(vertices, repeat=3):
		try:
			current_shortest = graph[i][j]
		except KeyError:
			current_shortest = inf
		try:
			new_path = graph[i][k] + graph[k][j]
		except KeyError:
			new_path = inf
		graph[i][j] = min(current_shortest, new_path)

def remove_stuck_valves(graph: dict[str, dict[str, int]], vertex_values: dict[str, int]):
	for vertex, value in vertex_values.copy().items():
		if value != 0:
			continue
		for end in graph[vertex].copy().keys():
			del graph[end][vertex]
		if vertex != "AA":
			del graph[vertex]
			del vertex_values[vertex]
			try:
				del graph["AA"][vertex]
			except KeyError:
				continue

def DFT(
	graph: dict[str, dict[str, int]],
	visited: dict[str, bool],
	start_vertex: str,
	vertex_values: dict[str, int],
	start_time: int
) -> int:
	max_pressure = 0
	stack = [(start_vertex, 0, max_pressure, start_time, visited)]
	while stack:
		current_vertex, pressure_production, current_pressure, time, visited = stack.pop()
		visited[current_vertex] = True
		max_pressure = max(max_pressure, current_pressure + pressure_production * time)
		for adjacent in graph[current_vertex].keys():
			if not visited[adjacent] and time > graph[current_vertex][adjacent]:
				stack.append((
					adjacent,
					pressure_production + vertex_values[adjacent],
					current_pressure + pressure_production * (graph[current_vertex][adjacent] + 1),
					time - graph[current_vertex][adjacent] - 1,
					visited.copy()
				))
	return max_pressure

def DFT_pairs(
	graph: dict[str, dict[str, int]],
	visited: dict[str, bool],
	start_vertex: str,
	vertex_values: dict[str, int],
	start_time: int
) -> int:
	max_pressure = 0
	stack = [(start_vertex, 0, max_pressure, start_time, visited)]
	while stack:
		current_vertex, pressure_production, current_pressure, time, visited = stack.pop()
		visited[current_vertex] = True
		max_pressure = max(
			max_pressure,
			current_pressure + pressure_production * time + DFT(
				graph,
				visited.copy(),
				start_vertex,
				vertex_values,
				start_time
			)
		)
		for adjacent in graph[current_vertex].keys():
			if not visited[adjacent] and time > graph[current_vertex][adjacent]:
				stack.append((
					adjacent,
					pressure_production + vertex_values[adjacent],
					current_pressure + pressure_production * (graph[current_vertex][adjacent] + 1),
					time - graph[current_vertex][adjacent] - 1,
					visited.copy()
				))
	return max_pressure

_input = "./2022/Day16/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
