from heapq import nlargest

def count(_input: str) -> int:
	with open(_input, "r", encoding="UTF-8") as file:
		return sum(nlargest(3, map(sum, (map(int, x.split("\n")) for x in file.read().split("\n\n")))))
 
print(count("./2022/Day1/input.txt"))
