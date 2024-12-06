def day4(input: str):
    total = 0
    grid_input = [list(line) for line in input.splitlines()]
    total += find_horizontal(input) + find_vertical(grid_input) + find_diagonal(grid_input)
    return total

def find_horizontal(input: str) -> int:
    return input.count("XMAS") + input.count("SAMX")



def find_vertical(grid_input: str) -> int:
    count = 0
    for row_index in range(len(grid_input) - 3):
        for column_index in range(len(grid_input[0])):
            if grid_input[row_index][column_index] == "X" and grid_input[row_index + 1][column_index] == "M" and grid_input[row_index + 2][column_index] == "A" and grid_input[row_index + 3][column_index] == "S":
                count += 1
            elif grid_input[row_index][column_index] == "S" and grid_input[row_index + 1][column_index] == "A" and grid_input[row_index + 2][column_index] == "M" and grid_input[row_index + 3][column_index] == "X":
                count += 1
    return count

def find_diagonal(grid_input: str) -> int:
    return 0


with open(r"C:\Users\nayna\Documents\Aoc-2024\input_04") as input_file:
    input = input_file.read()

print(day4(input))
