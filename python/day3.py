import re

def day3(input: str):
    cleaned = re.findall(r"(?:do\(\)|don't\(\)|mul\(\d+,\d+\))", input)
    nicened: list[list[int] | str] = [list(map(lambda x: int(x), element[4:-1].split(","))) if element != "do()" and element != "don't()" else element for element in cleaned]
    total = 0
    dont_flagged = False
    for elem in nicened:
        if elem == "don't()":
            dont_flagged = True
        elif elem == "do()":
            dont_flagged = False
        elif dont_flagged == False:
            total += elem[0] * elem[1] 

    return total

with open(r"C:\Users\nayna\Documents\Aoc-2024\input_03") as input_file:
    input = input_file.read()

print(day3(input))