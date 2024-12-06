use core::num;
use std::fmt::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()>{
    let file: File = File::open("../input_02.txt")?;
    let reader: BufReader<File> = BufReader::new(file);
    let lines: io::Lines<BufReader<File>>= reader.lines();
    // Process lines
    let sep_lines: Vec<Vec<i16>> = lines
        .map(|line| {
            line.unwrap() // Panic on error (use line.ok() for safe handling)
                .split(' ') // Split the line
                .map(|word| word.parse().unwrap()) // Convert &str to i32
                .collect()
        })
        .collect();
    let mut safe_report: i16 = 0;
    for line in sep_lines.iter(){
        if line[0] < line[1]{
            // Increasing
            safe_report += check_increasing(line).unwrap();
        }
        else if line[0] > line[1] {
            // Decreasing
            safe_report += check_decreasing(line).unwrap();
        }
    }
    print!("{}", safe_report);
    Ok(())
}

fn check_decreasing(line: &Vec<i16>) -> Result<i16, Error>{
    let mut acceptable_pairs = 0;

    for level_pair in line.windows(2){
        if level_pair[0] > level_pair[1]{
            if (level_pair[0] - level_pair[1]).abs() >= 1 && (level_pair[0] - level_pair[1]).abs() <= 3{
                acceptable_pairs += 1;
            } else{ 
                let first_dampened_line = check_increasing(&line.iter().filter(|x| **x != level_pair[0]).cloned().collect::<Vec<i16>>()).unwrap();
                let second_dampened_line = check_increasing(&line.iter().filter(|x| **x != level_pair[1]).cloned().collect::<Vec<i16>>()).unwrap();
                if first_dampened_line == 1 || second_dampened_line == 1{
                    return Ok(1)
                } else {
                    return Ok(0)
                }
                }
        } else {
            let first_dampened_line = check_increasing(&line.iter().filter(|x| **x != level_pair[0]).cloned().collect::<Vec<i16>>()).unwrap();
            let second_dampened_line = check_increasing(&line.iter().filter(|x| **x != level_pair[1]).cloned().collect::<Vec<i16>>()).unwrap();
            if first_dampened_line == 1 || second_dampened_line == 1{
                return Ok(1)
            } else {
                return Ok(0)
            }
        }  
    }   
    if acceptable_pairs == line.windows(2).len() {
        return Ok(1);
    } else {
        return Ok(0)
    }
}

fn check_increasing(line: &Vec<i16>) -> Result<i16, Error>{
    let mut acceptable_pairs = 0;

    for level_pair in line.windows(2){
        if level_pair[0] < level_pair[1]{
            if (level_pair[0] - level_pair[1]).abs() >= 1 && (level_pair[0] - level_pair[1]).abs() <= 3{
                acceptable_pairs += 1;
            } 
            else{ 
                let first_dampened_line = check_increasing(&line.iter().filter(|x| **x != level_pair[0]).cloned().collect::<Vec<i16>>()).unwrap();
                let second_dampened_line = check_increasing(&line.iter().filter(|x| **x != level_pair[1]).cloned().collect::<Vec<i16>>()).unwrap();
                if first_dampened_line == 1 || second_dampened_line == 1{
                    return Ok(1)
                } else {
                    return Ok(0)
                }
                }
        } else {
            let first_dampened_line = check_increasing(&line.iter().filter(|x| **x != level_pair[0]).cloned().collect::<Vec<i16>>()).unwrap();
            let second_dampened_line = check_increasing(&line.iter().filter(|x| **x != level_pair[1]).cloned().collect::<Vec<i16>>()).unwrap();
            if first_dampened_line == 1 || second_dampened_line == 1{
                return Ok(1)
            } else {
                return Ok(0)
            }
        }  
    }   
    if acceptable_pairs == line.windows(2).len() {
        return Ok(1);
    } else {
        return Ok(0)
    }
}