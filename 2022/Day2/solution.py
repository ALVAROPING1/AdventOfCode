def part1(_input: str) -> int:
    letter2value = {"A":0, "B":1, "C":2, "X":0, "Y":1, "Z":2}
    with open(_input, "r", encoding="UTF-8") as file:
        return sum(letter2value[game[2]] + 1 + ((letter2value[game[2]] - letter2value[game[0]] + 1) % 3) * 3 for game in file.readlines())

def part2(_input: str) -> int:
    letter2value = {"A":0, "B":1, "C":2, "X":0, "Y":1, "Z":2}
    with open(_input, "r", encoding="UTF-8") as file:
        return sum(letter2value[game[2]] * 3 + ((letter2value[game[0]] + letter2value[game[2]] - 1) % 3) + 1 for game in file.readlines())

_input = "./2022/Day2/input.txt"

print("Part 1:", part1(_input))
print("Part 2:", part2(_input))
