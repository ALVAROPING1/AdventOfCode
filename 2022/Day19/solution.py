from math import ceil
from dataclasses import dataclass
from heapq import heappop, heappush
from itertools import count
from typing import Iterator, NamedTuple, TypeAlias

Recipe: TypeAlias = tuple[int, int, int]

def part1(_input: str) -> int:
	quality_level_total = 0
	start_time = 24
	with open(_input, "r", encoding="UTF-8") as file:
		for index, line in enumerate(file.readlines()):
			quality_level_total += (index + 1) * Blueprint.from_line(line).max_geodes(start_time)
	return quality_level_total

def part2(_input: str) -> int:
	quality_level_total = 1
	start_time = 32
	with open(_input, "r", encoding="UTF-8") as file:
		for line in file.readlines()[:3]:
			quality_level_total *= Blueprint.from_line(line).max_geodes(start_time)
	return quality_level_total

@dataclass
class Blueprint:
	recipes: tuple[Recipe, Recipe, Recipe, Recipe]
	max_robots: tuple[int, int, int, int]

	@classmethod
	def from_line(cls, input_line) -> "Blueprint":
		words = input_line.split()[6::3]
		recipes = (
            (int(words[0]), 0, 0),
            (int(words[2]), 0, 0),
            (int(words[4]), int(words[5]), 0),
            (int(words[7]), 0, int(words[8])),
		)
		return cls(recipes, (*(max(resource) for resource in zip(*recipes)), 0))

	def max_geodes(self, start_time: int) -> int:
		tiebreaker = count()
		queue: list[tuple[int, int, int, int, int, State]] = []
		def append(state: State):
			heappush(queue, (*state.priority, next(tiebreaker), state))

		max_geodes = 0
		state = State(start_time)
		append(state)
		seen_states = {state}
		while queue:
			*_, state = heappop(queue)
			for next_state in state.traverse(self):
				if next_state in seen_states or next_state.max_geodes_upper_bound <= max_geodes:
					continue
				max_geodes = max(max_geodes, next_state.max_geodes)
				seen_states.add(next_state)
				append(next_state)
		return max_geodes

class State(NamedTuple):
	time: int
	robots: tuple[int, int, int, int] = (1, 0, 0, 0)
	resources: tuple[int, int, int, int] = (0, 0, 0, 0)

	@property
	def max_geodes(self) -> int:
		return self.resources[3]

	@property
	def max_geodes_upper_bound(self) -> int:
		return self.max_geodes + (self.time * (self.time - 1)) / 2

	@property
	def priority(self) -> tuple[int, int, int, int]:
		return (*(-resource for resource in self.resources[:0:-1]), self.time)

	def is_resource_infinite(self, blueprint: Blueprint, resource: int) -> bool:
		return (
			blueprint.max_robots[resource]
			and self.robots[resource] * (self.time - 1) + self.resources[resource]
			>= blueprint.max_robots[resource] * self.time
		)

	def traverse(self, blueprint: Blueprint) -> Iterator["State"]:
		for resource_type, robot_recipe in zip(range(4), blueprint.recipes):
			if (
				self.is_resource_infinite(blueprint, resource_type) or
				(not all(bool(robots) for robots, required in zip(self.robots, robot_recipe) if required))
			):
				continue
			time_required = 1 + max(
				0 if available >= required else ceil((required - available) / produced)
				for available, required, produced in zip(self.resources, robot_recipe, self.robots)
				if required
			)
			if time_required >= self.time:
				continue

			new_resources = [
				10000 if self.is_resource_infinite(blueprint, resource) else
				available - required + produced * time_required
				for available, required, produced, resource in zip(
					self.resources, (*robot_recipe, 0), self.robots, range(4)
				)
			]
			if resource_type != 3:
				new_robots = list(self.robots)
				new_robots[resource_type] += 1
			else:
				new_robots = self.robots
				new_resources[3] += self.time - time_required
			yield State(self.time - time_required, tuple(new_robots), tuple(new_resources))

_input = "./2022/Day19/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
