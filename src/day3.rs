use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MUL_RE: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    static ref OP_RE: Regex = Regex::new(
        r"(?:do\(\))|(?:don't\(\))|(?:mul\((\d{1,3}),(\d{1,3})\))").unwrap();
}

fn read_lines(source: Option<String>) -> Vec<String> {
    match source {
        None => std::io::stdin().lock().lines().map(Result::unwrap).collect(),
        Some(name) => BufReader::new(File::open(name).unwrap()).lines().map(Result::unwrap).collect(),
    }
}

fn count_muls(lines: Vec<String>) -> i32 {
    let mut result = 0;
    for line in lines {
        for capture in MUL_RE.captures_iter(line.as_str()) {
            let (full, [l, r]) = capture.extract();
            println!("Matched {}", full);
            result += l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap();
        }
    }
    result
}

fn count_enabled_muls(lines: Vec<String>) -> i32 {
    let mut result = 0;
    let mut enabled = true;
    for line in lines {
        for capture in OP_RE.captures_iter(line.as_str()) {
            let full: &str = capture.get(0).unwrap().as_str();
            if full == "do()" {
                enabled = true;
            } else if full == "don't()" {
                enabled = false;
            } else {
                let l = capture.get(1).unwrap().as_str();
                let r = capture.get(2).unwrap().as_str();
                if enabled {
                    result += l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap();
                }
            }
        }
    }
    result
}

pub fn day3(source: Option<String>) -> i32 {
    let lines = read_lines(source);
    count_muls(lines)
}

pub fn day3b(source: Option<String>) -> i32 {
    let lines = read_lines(source);
    count_enabled_muls(lines)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(day3(Some("data/day3_example.txt".to_string())), 161);
    }

    #[test]
    fn test_test() {
        assert_eq!(day3(Some("data/day3_test.txt".to_string())), 165225049);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(day3b(Some("data/day3_example_b.txt".to_string())), 48);
    }
    
    #[test]
    fn test_test_b() {
        assert_eq!(day3b(Some("data/day3_test.txt".to_string())), 108830766);
    }
}