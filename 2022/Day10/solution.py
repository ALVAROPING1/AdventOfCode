def part1(_input: str) -> int:
	current_cycle = 1
	registerX = 1
	total_signal_strength = 0
	with open(_input, "r", encoding="UTF-8") as file:
		for instruction in file.readlines():
			current_cycle += 1
			if (current_cycle - 20) % 40 == 0:
				total_signal_strength += current_cycle * registerX
			if instruction[0] == "a":
				current_cycle += 1
				registerX += int(instruction[5:])
				if (current_cycle - 20) % 40 == 0:
					total_signal_strength += current_cycle * registerX
	return total_signal_strength

def part2(_input: str) -> list[str]:
	current_cycle = 1
	registerX = 1
	rendered_image = ["#", "", "", "", "", ""]
	with open(_input, "r", encoding="UTF-8") as file:
		for instruction in file.readlines():
			current_cycle += 1
			if current_cycle > 240:
				break
			draw_pixel(rendered_image, current_cycle, registerX)
			if instruction[0] == "a":
				current_cycle += 1
				if current_cycle > 240:
					break
				registerX += int(instruction[5:])
				draw_pixel(rendered_image, current_cycle, registerX)
	return rendered_image

def draw_pixel(image: list[str], current_cycle: int, current_position: int) -> str:
	image[(current_cycle - 1)//40] += "#" if current_position <= current_cycle % 40 <= current_position + 2 else "."

_input = "./2022/Day10/input.txt"

print("Part 1:", part1(_input))
print("Part 2:")
for line in part2(_input):
	print(line)
