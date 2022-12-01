def count(_input: str) -> int:
	maximum = [0,0,0]
	with open(_input, "r", encoding="UTF-8") as file:
		for element in file.read().split("\n\n"):
			current = sum(int(x) for x in element.split("\n"))
			for i in range(2, -1, -1):
				maximum[i], current = max(current, maximum[i]), min(current, maximum[i]) 
	return sum(maximum)

print(count("./2022/Day1/input.txt"))
