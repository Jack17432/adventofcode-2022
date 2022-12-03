use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn calorie_counting(file_name: &str) -> i128 {
    let file =  File::open(file_name)
        .expect("file not found");

    let mut largest = 0;
    let mut curr = 0;

    let buf_reader = BufReader::new(file);
    for result_line in buf_reader.lines() {
        match result_line {
            Ok(line) => {
                if line.is_empty() {
                    if curr > largest {
                        largest = curr;
                    }
                    curr = 0;
                }
                else {
                    curr += line.parse::<i128>().unwrap();
                }
            }
            Err(..) => break,
        }
    }

    largest
}

pub fn calorie_counting_top3(file_name: &str) -> i128 {
    let file = File::open(file_name)
        .expect("file not found");

    let mut largest = 0;
    let mut largest2 = 0;
    let mut largest3 = 0;
    let mut curr = 0;

    let buf_reader = BufReader::new(file);
    for result_line in buf_reader.lines() {
        match result_line {
            Ok(line) => {
                if line.is_empty() {
                    if curr > largest{
                        largest3 = largest2;
                        largest2 = largest;
                        largest = curr;
                    } else if curr > largest2 {
                        largest3 = largest2;
                        largest2 = curr;
                    } else if curr > largest3 {
                        largest3 = curr;
                    }
                    curr = 0;
                }
                else {
                    curr += line.parse::<i128>().unwrap();
                }
            }
            Err(..) => break,
        }
    }

    largest + largest2 + largest3
}