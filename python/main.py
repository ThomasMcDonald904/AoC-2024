from collections import Counter


with open("input_01") as input_file:
    lines = input_file.readlines()

def day_01(lines: list()):
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

print(day_01(lines))