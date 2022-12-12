import heapq
from typing import NamedTuple
from math import inf

def part1(_input: str) -> int:
	graph = []
	with open(_input, "r", encoding="UTF-8") as file:
		for indexX, line in enumerate(file.readlines()):
			current_line = []
			for indexY, char in enumerate(line[:-1]):
				if char == "S":
					end = Node(indexX, indexY)
					char = "a"
				elif char == "E":
					start = Node(indexX, indexY)
					char = "z"
				current_line.append(ord(char))
			graph.append(current_line)
	return a_star(start, end, graph)

def part2(_input: str) -> int:
	graph = []
	end = []
	with open(_input, "r", encoding="UTF-8") as file:
		for indexX, line in enumerate(file.readlines()):
			current_line = []
			for indexY, char in enumerate(line[:-1]):
				if char == "S":
					char = "a"
				if char == "a":
					end.append(Node(indexX, indexY))
				elif char == "E":
					start = Node(indexX, indexY)
					char = "z"
				current_line.append(ord(char))
			graph.append(current_line)
	return a_star_multiple_goals(start, end, ord("a"), graph)

class Node(NamedTuple):
	x: int
	y: int

	def get_heuristic(self, goal:"Node") -> int:
		return abs(goal.x - self.x) + abs(goal.y - self.y)

	def get_heuristic_multiple_goals(self, goal: tuple["Node"]) -> int:
		return min(abs(node.x - self.x) + abs(node.y - self.y) for node in goal)

	def get_neighbors(self, graph: tuple[tuple[int]]) -> list["Node"]:
		out = []
		if self.x > 0 and graph[self.x-1][self.y] >= graph[self.x][self.y] - 1:
			out.append(Node(self.x - 1, self.y))
		if self.x < len(graph)-1 and graph[self.x+1][self.y] >= graph[self.x][self.y] - 1:
			out.append(Node(self.x + 1, self.y))
		if self.y > 0 and graph[self.x][self.y-1] >= graph[self.x][self.y] - 1:
			out.append(Node(self.x, self.y - 1))
		if self.y < len(graph[0])-1 and graph[self.x][self.y+1] >= graph[self.x][self.y] - 1:
			out.append(Node(self.x, self.y + 1))
		return out

def a_star(start: Node, goal: Node, graph: tuple[tuple[int]]) -> int:
	openSet: list[tuple[int, Node]] = []
	gScore = [[inf for _ in range(len(graph[0]))] for _ in range(len(graph))]
	gScore[start.x][start.y] = 0
	fScore = [[inf for _ in range(len(graph[0]))] for _ in range(len(graph))]
	fScore[start.x][start.y] = start.get_heuristic(goal)
	heapq.heappush(openSet, (fScore[start.x][start.y], start))
	while openSet:
		current_score, current = heapq.heappop(openSet)
		if current == goal:
			return current_score
		for neighbor in current.get_neighbors(graph):
			tentative_gScore = gScore[current.x][current.y] + 1
			if tentative_gScore < gScore[neighbor.x][neighbor.y]:
				gScore[neighbor.x][neighbor.y] = tentative_gScore
				fScore[neighbor.x][neighbor.y] = tentative_gScore + neighbor.get_heuristic(goal)
				queue_element = (fScore[neighbor.x][neighbor.y], neighbor)
				if queue_element not in openSet:
					heapq.heappush(openSet, queue_element)
	return None

def a_star_multiple_goals(start: Node, goal: tuple[Node], goal_value: int, graph: tuple[tuple[int]]) -> int:
	openSet: list[tuple[int, Node]] = []
	gScore = [[inf for _ in range(len(graph[0]))] for _ in range(len(graph))]
	gScore[start.x][start.y] = 0
	fScore = [[inf for _ in range(len(graph[0]))] for _ in range(len(graph))]
	fScore[start.x][start.y] = start.get_heuristic_multiple_goals(goal)
	heapq.heappush(openSet, (fScore[start.x][start.y], start))
	while openSet:
		current_score, current = heapq.heappop(openSet)
		if graph[current.x][current.y] == goal_value:
			return current_score
		for neighbor in current.get_neighbors(graph):
			tentative_gScore = gScore[current.x][current.y] + 1
			if tentative_gScore < gScore[neighbor.x][neighbor.y]:
				gScore[neighbor.x][neighbor.y] = tentative_gScore
				fScore[neighbor.x][neighbor.y] = tentative_gScore + neighbor.get_heuristic_multiple_goals(goal)
				queue_element = (fScore[neighbor.x][neighbor.y], neighbor)
				if queue_element not in openSet:
					heapq.heappush(openSet, queue_element)
	return None

_input = "./2022/Day12/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
