from itertools import takewhile

def part1(_input: str) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		stack_str, moves_str = file.read().split("\n\n")
		stack_lines = stack_str.split("\n")[:-1]
		stacks = [[line[4*i + 1] for line in takewhile(charIsNotSpace(4*i + 1), reversed(stack_lines))] for i in range(int(stack_str[-2]))]
		for move in moves_str.split("\n")[:-1]:
			stack_number = int(move[-6]) - 1
			stack_size = int(move[-13 - (len(move) > 18):-12])
			stacks[int(move[-1])-1].extend(stacks[stack_number][:-stack_size-1:-1])
			del stacks[stack_number][-stack_size:]
		return "".join(stack[-1] for stack in stacks)

def part2(_input: str) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		stack_str, moves_str = file.read().split("\n\n")
		stack_lines = stack_str.split("\n")[:-1]
		stacks = [[line[4*i + 1] for line in takewhile(charIsNotSpace(4*i + 1), reversed(stack_lines))] for i in range(int(stack_str[-2]))]
		for move in moves_str.split("\n")[:-1]:
			stack_number = int(move[-6]) - 1
			stack_size = int(move[-13 - (len(move) > 18):-12])
			stacks[int(move[-1])-1].extend(stacks[stack_number][-stack_size:])
			del stacks[stack_number][-stack_size:]
		return "".join(stack[-1] for stack in stacks)

def charIsNotSpace(pos: int) -> bool:
	return lambda line: line[pos] != " "

_input = "./2022/Day5/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
