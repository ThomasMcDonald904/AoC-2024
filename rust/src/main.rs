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
    let mut number_of_problems = 0;
    for level_pair in line.windows(2){
        if level_pair[0] > level_pair[1]{
            if (level_pair[0] - level_pair[1]).abs() >= 1 && (level_pair[0] - level_pair[1]).abs() <= 3{
                acceptable_pairs += 1;
            } else {
                print!("{} level pair is fucked counting", level_pair.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("~"));
                number_of_problems += 1;
            }}
         else {
            print!("{} level pair is increasing", level_pair.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("~"));
            number_of_problems += 1;

        
    }   }
    if acceptable_pairs >= line.windows(2).len()-1{
        println!("Line: {} is safe. Used Problem dampener is {}", line.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("-"), number_of_problems != 0);
        return Ok(1)
    } else {
        println!("Line: {} is unsafe", line.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("-"));
        return Ok(0)
    }
}

fn check_increasing(line: &Vec<i16>) -> Result<i16, Error>{
    let mut acceptable_pairs = 0;
    let mut number_of_problems = 0;

    for level_pair in line.windows(2){
        if level_pair[0] < level_pair[1]{
            if (level_pair[0] - level_pair[1]).abs() >= 1 && (level_pair[0] - level_pair[1]).abs() <= 3{
                acceptable_pairs += 1;
            } 
            else{ 
                print!("{} level pair is fucked counting", level_pair.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("~"));
                number_of_problems += 1;
                }
        } else {
            print!("{} level pair is decreasing", level_pair.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("~"));
            number_of_problems += 1;

        }  
    }   
    if number_of_problems <= 1 {
        println!("Line: {} is safe. Used Problem dampener is {}", line.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("-"), number_of_problems != 0);
        return Ok(1)
    } else {
        println!("Line: {} is unsafe", line.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("-"));
        return Ok(0)
    }
}