use std::io::{self, BufRead};
use std::vec::Vec;

pub fn read_all_records() -> Vec<Vec<String>> {
    let mut result = Vec::<Vec::<String>>::new();
    for line in io::stdin().lock().lines() {
        let content = line.unwrap();
        let record: Vec<String> = content.split_whitespace();
    }
    result
}