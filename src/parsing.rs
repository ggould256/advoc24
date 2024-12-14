use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use regex::Regex;

pub fn read_regex_records(source: Option<String>, record_regex: Regex) -> Vec<Vec<String>> {
    let input = read_one_string(source);
    let mut result = Vec::new();
    for record_capture in record_regex.captures_iter(input.as_str()) {
        let mut fields: Vec<String> = Vec::new();
        for sub_match in record_capture.iter() {
            let Some(sub_match) = sub_match else { continue };
            fields.push(sub_match.as_str().to_string());
        }
        result.push(fields);
    }
    result
}

pub fn read_one_string(source: Option<String>) -> String {
    match source {
        None => {
            let mut buf = String::new();
            std::io::stdin().lock().read_to_string(&mut buf).unwrap();
            buf
        }
        Some(name) => {
            let mut buf = String::new();
            BufReader::new(File::open(name).unwrap())
                .read_to_string(&mut buf).unwrap();
            buf
        }
    }
}

pub fn read_lines(source: Option<String>) -> Vec<String> {
    match source {
        None => std::io::stdin().lock().lines().map(Result::unwrap).collect(),
        Some(name) => BufReader::new(File::open(name).unwrap()).lines().map(Result::unwrap).collect(),
    }
}

pub fn read_all_records(source: Option<String>) -> Vec<Vec<String>> {
    match source {
        None => read_all_records_from_readable(std::io::stdin().lock()),
        Some(name) => read_all_records_from_readable(
            BufReader::new(File::open(name).unwrap())),
    }
}

fn read_all_records_from_readable<T>(readable: T) -> Vec<Vec<String>>
where
    T: BufRead,
{
    let mut result: Vec<Vec<String>> = Vec::<Vec<String>>::new();
    for line in readable.lines() {
        let content = line.unwrap();
        let record: Vec<String> = content.split_whitespace().map(String::from).collect();
        result.push(record);
    }
    result
}

pub fn parse_as_ii(input: Vec<Vec<String>>) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();
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

pub fn parse_as_i32s(input: Vec<Vec<String>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    for input_record in input {
        let mut record: Vec<i32> = Vec::new();
        for e in input_record {
            record.push(e.parse::<i32>().unwrap())
        }
        result.push(record);
    }
    result
}

pub fn stride_text(input: &str, start: usize, stride: usize) -> String {
    let mut line_iter = input.chars();
    let mut line = String::new();
    line += &line_iter.nth(start).unwrap().to_string();
    loop {
        match line_iter.nth(stride - 1) {
            None => {
                break;
            }
            Some(c) => {
                line += &c.to_string();
            }
        }
    }
    line
}
