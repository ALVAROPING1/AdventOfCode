from math import log as BSF
from math import floor
from operator import ior as bitwiseOR, iand as bitwiseAND
from functools import reduce
from re import finditer

def part1(_input: str) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		return floor(sum(BSF(
				reduce(bitwiseOR, map(char2pow2, line[:len(line)//2])) &
				reduce(bitwiseOR, map(char2pow2, line[len(line)//2:-1])),
				2
			) + 1 for line in file.readlines()
		))

def part2(_input: str) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		return floor(sum(BSF(
				reduce(
					bitwiseAND,
					(reduce(bitwiseOR, map(char2pow2, backpack)) for backpack in group[0][:-1].split("\n"))
				)
				, 2
			) + 1 for group in finditer(r"(\w+\n){3}", file.read())
		))

def char2pow2(_in: str) -> int:
	return 2 ** (ord(_in)-97 if ord(_in) > 96 else ord(_in)-39)

def BSF64(x: int) -> int:
	"""Get the index of the rightmost 1 using bit hacks"""
	if x == 0:
		return -1
	x = x & -x
	count = 0
	if (x & 18446744069414584320) != 0: count += 32 # 0xffffffff00000000
	if (x & 18446462603027742720) != 0: count += 16 # 0xffff0000ffff0000
	if (x & 18374966859414961920) != 0: count += 8  # 0xff00ff00ff00ff00
	if (x & 17361641481138401520) != 0: count += 4  # 0xf0f0f0f0f0f0f0f0
	if (x & 14757395258967641292) != 0: count += 2  # 0xcccccccccccccccc
	if (x & 12297829382473034410) != 0: count += 1  # 0xaaaaaaaaaaaaaaaa
	return count

_input = "./2022/Day3/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
