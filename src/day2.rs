use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn stage1(file_name: &str) -> i128{
    let file = File::open(file_name)
        .expect("file not found");

    let mut score = 0;

    let buf_reader = BufReader::new(file);
    for result_line in buf_reader.lines() {
        match result_line {
            Ok(line) => {
                score += line.as_bytes()[2] as i128 - 87 ;
                match line.as_str() {
                    "A Y" => score += 6,
                    "B Z" => score += 6,
                    "C X" => score += 6,
                    "A X" => score += 3,
                    "B Y" => score += 3,
                    "C Z" => score += 3,
                    _ => {}
                }
            },
            Err(..) => break,
        }
    }

    score
}

pub fn stage2(file_name: &str) -> i128 {
    let file = File::open(file_name)
        .expect("file not found");

    let mut score = 0;

    let buf_reader = BufReader::new(file);
    for result_line in buf_reader.lines() {
        match result_line {
            Ok(line) => {
                match line.as_bytes()[2] {
                    90 => score += 6,
                    89 => score += 3,
                    _ => {}
                }
                match line.as_bytes()[2] {
                    88 => score += (line.as_bytes()[0] as i128 - 66).rem_euclid(3) + 1,
                    89 => score += line.as_bytes()[0] as i128 - 64,
                    _ => score += (line.as_bytes()[0] as i128 - 64).rem_euclid(3) + 1
                }

            },
            Err(..) => break,
        }
    }

    score
}