def count(_input: str) -> int:
	maximum = [0,0,0]
	current = 0
	with open(_input, "r", encoding="UTF-8") as file:
		for element in file.readlines():
			if element == "\n":
				if current > maximum[0]:
					if current > maximum[1]: 
						if current > maximum[2]: maximum = maximum[1:] + [current]
						else: maximum[0:2] = [maximum[1], current]
					else: maximum[0] = current
				current = 0
			else:
				current += int(element)
	return sum(maximum)

print(count("./Day1/input.txt"))