use std::collections::{HashMap, HashSet};
use std::iter::zip;
use std::vec::Vec;

use crate::parsing::{read_all_records, parse_as_ii};

fn sorted_error_sum(records: Vec<(i32, i32)>) -> i32 {
    let mut result = 0;
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = records.iter().cloned().unzip();
    left.sort();
    right.sort();
    for (x, y) in zip(left, right) {
        result += (x - y).abs();
    }
    result
}

fn similarity_score(records: Vec<(i32, i32)>) -> i32 {
    let mut result: i32 = 0;
    let (left, right): (Vec<i32>, Vec<i32>) = records.iter().cloned().unzip();
    let keys: HashSet<i32> =
        &HashSet::from_iter(left.iter().cloned()) | &HashSet::from_iter(left.iter().cloned());
    let mut left_counts: HashMap<i32, i32> = HashMap::new();
    let mut right_counts: HashMap<i32, i32> = HashMap::new();
    for l in left {
        left_counts.insert(l, left_counts.get(&l).unwrap_or(&0) + 1);
    }
    for r in right {
        right_counts.insert(r, right_counts.get(&r).unwrap_or(&0) + 1);
    }
    for k in keys {
        let score: i32 = k * left_counts.get(&k).unwrap_or(&0) * right_counts.get(&k).unwrap_or(&0);
        result += score;
    }
    result
}

pub fn day1(source: Option<String>) -> i32 {
    let records = read_all_records(source);
    let parsed = parse_as_ii(records);
    let result = sorted_error_sum(parsed);
    result
}

pub fn day1b(source: Option<String>) -> i32 {
    let records = read_all_records(source);
    let parsed = parse_as_ii(records);
    let result = similarity_score(parsed);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let example_records = read_all_records(Some("data/day1_example.txt".to_string()));
        let result = sorted_error_sum(parse_as_ii(example_records));
        assert_eq!(result, 11);
    }

    #[test]
    fn test_test_1() {
        let example_records = read_all_records(Some("data/day1_test.txt".to_string()));
        let result = sorted_error_sum(parse_as_ii(example_records));
        assert_eq!(result, 1319616);
    }

    #[test]
    fn test_example_1b() {
        let example_records = read_all_records(Some("data/day1_example.txt".to_string()));
        let result = similarity_score(parse_as_ii(example_records));
        assert_eq!(result, 31);
    }

    #[test]
    fn test_test_1b() {
        let example_records = read_all_records(Some("data/day1_test.txt".to_string()));
        let result = similarity_score(parse_as_ii(example_records));
        assert_eq!(result, 27267728);
    }
}
