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
    let mut has_tolerated_error: bool = false;
    for level_pair in line.windows(2){
        if level_pair[0] > level_pair[1]{
            if (level_pair[0] - level_pair[1]).abs() >= 1 && (level_pair[0] - level_pair[1]).abs() <= 3{
                acceptable_pairs += 1;
            } 
            else if ! has_tolerated_error {
                    acceptable_pairs += 1;
                    has_tolerated_error = true;
            }
        } else {
            if ! has_tolerated_error{
                acceptable_pairs += 1;
                has_tolerated_error = true;
            }
        }  
    }   
    if (acceptable_pairs == line.windows(2).len()) || (has_tolerated_error && acceptable_pairs == line.windows(2).len() + 1){
        println!("Line: {} is safe. Used Problem dampener is {}", line.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("-"), has_tolerated_error);
        return Ok(1)
    } else {
        println!("Line: {} is unsafe", line.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("-"));
        return Ok(0)
    }
}

fn check_increasing(line: &Vec<i16>) -> Result<i16, Error>{
    let mut acceptable_pairs = 0;
    let mut has_tolerated_error: bool = false;
    for level_pair in line.windows(2){
        if level_pair[0] < level_pair[1]{
            if (level_pair[0] - level_pair[1]).abs() >= 1 && (level_pair[0] - level_pair[1]).abs() <= 3{
                acceptable_pairs += 1;
            } 
            else if ! has_tolerated_error {
                    acceptable_pairs += 1;
                    has_tolerated_error = true;
            }
        } else {
            if ! has_tolerated_error{
                acceptable_pairs += 1;
                has_tolerated_error = true;
            }
        }  
    }   
    if (acceptable_pairs == line.windows(2).len()) || (has_tolerated_error && acceptable_pairs == line.windows(2).len() + 1){
        println!("Line: {} is safe. Used Problem dampener is {}", line.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("-"), has_tolerated_error);
        return Ok(1)
    } else {
        println!("Line: {} is unsafe", line.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("-"));
        return Ok(0)
    }
}