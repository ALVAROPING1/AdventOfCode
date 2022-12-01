def count(_input: str) -> int:
	maximum = 0
	current = 0
	with open(_input, "r", encoding="UTF-8") as file:
		for element in file.readlines():
			if element == "\n":
				maximum = max(maximum, current)
				current = 0
			else:
				current += int(element)
		return maximum

print(count("./Day1/input.txt"))
