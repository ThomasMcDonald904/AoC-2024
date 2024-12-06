def day4(input: str):
    total = 0
    total += find_horizontal(input) + find_vertical(input) + find_diagonal(input)
    return total

def find_horizontal(input: str) -> int:
    return 0


def find_vertical(input: str) -> int:
    return 0

def find_diagonal(input: str) -> int:
    return 0


with open(r"C:\Users\nayna\Documents\Aoc-2024\input_04") as input_file:
    input = input_file.read()

print(day4(input))
