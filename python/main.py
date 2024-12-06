from collections import Counter
from typing import List

def day_01(lines: list):
    left_column = []
    right_column = []
    for line in lines:
        left_column.append(int(line.split("   ")[0]))
        right_column.append(int(line.split("   ")[1]))
    left_column.sort()
    right_column.sort()
   
    diff = 0
    for element in range(len(left_column)):
        diff += abs(left_column[element] - right_column[element])

    similarity_score = 0
    counter = Counter(right_column)
    for element in left_column:
        if element in counter.keys():
            similarity_score += counter[element] * element

    return diff, similarity_score
def day_02(lines: List[str]):
    lines_int = []
    for raw_line in lines:
        lines_int.append([int(i) for i in raw_line.split()])
    return check_security(lines_int)

def check_security(lines: List[List[int]]):
    counter = 0
    for line in lines:
        if is_safe(line):
            counter += 1
        else:
            if is_safe_with_dampener(line):
                counter += 1
    return counter

def is_safe(line: List[int]) -> bool:
    is_increasing = True
    is_decreasing = True
    for i in range(1, len(line)):
        diff = abs(line[i] - line[i - 1])
        if diff < 1 or diff > 3:
            return False
        if line[i] < line[i - 1]:
            is_increasing = False
        elif line[i] > line[i - 1]:
            is_decreasing = False
    
    return is_increasing or is_decreasing

def is_safe_with_dampener(line: List[int]) -> bool:
    for i in range(len(line)):
        new_line = line[:i] + line[i + 1:]
        if is_safe(new_line):
            return True
    return False

with open(r"C:\Users\nayna\Documents\Aoc-2024\input_02") as input_file:
    lines = input_file.readlines()

print(day_02(lines))