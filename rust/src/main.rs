use std::collections::HashMap;

fn day_01(lines: Vec<String>) -> (i32, i32) {
    let mut left_column: Vec<i32> = Vec::new();
    let mut right_column: Vec<i32> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split("   ").collect();
        left_column.push(parts[0].parse().unwrap());
        right_column.push(parts[1].parse().unwrap());
    }

    left_column.sort();
    right_column.sort();

    // Part One
    let mut diff = 0;
    for i in 0..left_column.len() {
        diff += (left_column[i] - right_column[i]).abs();
    }

    // Part Two
    let mut similarity_score = 0;
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for &element in &right_column {
        *counter.entry(element).or_insert(0) += 1;
    }

    for &element in &left_column {
        if let Some(&count) = counter.get(&element) {
            similarity_score += count * element;
        }
    }

    (diff, similarity_score)
}

fn day_02(lines: Vec<String>) -> i32 {
    let mut lines_int: Vec<Vec<i32>> = Vec::new();
    for raw_line in lines {
        let line: Vec<i32> = raw_line.split_whitespace()
                                     .map(|s| s.parse().unwrap())
                                     .collect();
        lines_int.push(line);
    }
    check_security(lines_int)
}

fn check_security(lines: Vec<Vec<i32>>) -> i32 {
    let mut counter = 0;
    for line in lines {
        if is_safe(&line) {
            counter += 1;
        } else if is_safe_with_dampener(&line) {
            counter += 1;
        }
    }
    counter
}

fn is_safe(line: &Vec<i32>) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 1..line.len() {
        let diff = (line[i] - line[i - 1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
        if line[i] < line[i - 1] {
            is_increasing = false;
        } else if line[i] > line[i - 1] {
            is_decreasing = false;
        }
    }

    is_increasing || is_decreasing
}

fn is_safe_with_dampener(line: &Vec<i32>) -> bool {
    for i in 0..line.len() {
        let mut new_line = line.clone();
        new_line.remove(i);
        if is_safe(&new_line) {
            return true;
        }
    }
    false
}

fn main() {
    let input_file = std::fs::read_to_string(r"C:\Users\nayna\Documents\Aoc-2024\input_02").expect("Failed to read file");
    let lines: Vec<String> = input_file.lines().map(|line| line.to_string()).collect();

    let result = day_02(lines);
    println!("{}", result);
}
