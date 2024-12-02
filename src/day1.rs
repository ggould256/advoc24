use std::io::{self, BufRead};
use std::vec::Vec;

fn read_all_records() -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::<Vec<String>>::new();
    for line in io::stdin().lock().lines() {
        let content = line.unwrap();
        let record: Vec<String> = content.split_whitespace().map(String::from).collect();
        result.push(record);
    }
    result
}

fn parse_records(input: Vec<Vec<String>>) -> Vec<(i32, i32)> {
    let mut result = Vec::<(i32, i32)>::new();
    for input_record in input {
        assert!(input_record.len() == 2);
        let record: (i32, i32) = (
            input_record[0].parse::<i32>().unwrap(),
            input_record[1].parse::<i32>().unwrap(),
        );
        result.push(record);
    }
    result
}

fn order_distance(records: Vec<(i32, i32)>) -> usize {
    let mut result = 0;
    let (left, right): (Vec<i32>, Vec<i32>) = records.iter().cloned().unzip();
    for i in 0..left.len() {
        let target_value = left[i];
        let target_location = right.iter().position(|&x| x == target_value).unwrap();
        let distance = i.abs_diff(target_location);
        result += distance;
    }
    result
}

pub fn day1() -> usize {
    let records = read_all_records();
    let parsed = parse_records(records);
    let result = order_distance(parsed);
    result
}
