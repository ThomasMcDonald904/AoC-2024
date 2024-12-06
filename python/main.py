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
   
    # Part One
    diff = 0
    for element in range(len(left_column)):
        diff += abs(left_column[element] - right_column[element])

    #Part Two
    similarity_score = 0
    counter = Counter(right_column)
    for element in left_column:
        if element in counter.keys():
            similarity_score += counter[element] * element

    return diff, similarity_score

with open("input_02") as input_file:
    lines = input_file.readlines()

def day_02(lines: list[str]):
    counter = 0
    lines_int = []
    for raw_line in lines:
        lines_int.append([int(i) for i in raw_line.split(" ")])
    return check_security(lines_int)

def check_security(lines: list[int]):
    counter = 0
    for line in lines:
        ascending_line = sorted(line)
        descending_line = sorted(line, reverse=True)
        asc_diff:List[int] = [line[i] - ascending_line[i] for i in range(len(line))]
        dec_diff:List[int] = [line[i] - descending_line[i] for i in range(len(line))]
        adj_diff = check_adjacent_diff(line)
        if adj_diff[0] == True and adj_diff[1] == False:
            if len(list(filter(lambda x:  x != 0, asc_diff))) <= 1 or len(list(filter(lambda x:  x != 0, dec_diff))) <= 1:
                counter += 1
        elif adj_diff[0] == True and adj_diff[1] is list:
            if len(list(filter(lambda x:  x != 0, adj_diff[1]))) == 0 or len(list(filter(lambda x:  x != 0, adj_diff[1]))) == 0:
                counter += 1
        
    return counter
                

    return counter

def check_adjacent_diff(levels:list[int]):
    for i in range(len(levels) - 1):
        diff = abs(levels[i] - levels[i + 1])
        if not (1 <= diff <= 3):
            left = check_adjacent_diff([x for x in levels if x != levels[i]])
            right = check_adjacent_diff([x for x in levels if x != levels[i + 1]])
            if left == (True, False):
                return True, sorted([x for x in levels if x != levels[i]])
            elif right == (True, False):
                return True, sorted([x for x in levels if x != levels[i+1]])
            else: 
                return False, False
            
    return True, False

print(day_02(lines))