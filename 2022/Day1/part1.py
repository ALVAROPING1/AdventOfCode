def count(_input: str) -> int:
	maximum = 0
	with open(_input, "r", encoding="UTF-8") as file:
		for element in file.read().split("\n\n"):
			maximum = max(maximum, sum(int(x) for x in element.split("\n")))
		return maximum

print(count("./2022/Day1/input.txt"))
