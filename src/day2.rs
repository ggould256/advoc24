use std::io;

use crate::parsing::{read_all_records, parse_as_i32s};

fn record_is_safe(record: Vec<i32>) -> bool {
    let differences: Vec<i32> =
        (0..record.len()-1).map(|i| record[i+1] - record[i]).collect();
    // Ascending or descending.
    if differences.iter().all(|&diff| diff > 0) || differences.iter().all(|&diff| diff < 0) {
        // Bounded.
        if differences.iter().all(|&diff| -3 <= diff && diff <= 3) {
            return true;
        }
    }
    false
}

fn total_safe(records: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for record in records {
        if record_is_safe(record) {
            result += 1;
        }
    }
    result
}

pub fn day2() -> i32 {
    let records = parse_as_i32s(
        read_all_records(io::stdin().lock()));
    total_safe(records)
}

pub fn day2b() -> i32 {
    let result = 0;
    result
}


#[cfg(test)]
mod tests {
    use io::BufReader;
    use std::fs::File;

    use super::*;

    #[test]
    fn test_example() {
        let f = File::open("data/day2_example.txt").unwrap();
        let reader = BufReader::new(f);
        let example_records = read_all_records(reader);
        let result = total_safe(parse_as_i32s(example_records));
        assert_eq!(result, 2);
    }

    #[test]
    fn test_test() {
        let f = File::open("data/day2_test.txt").unwrap();
        let reader = BufReader::new(f);
        let example_records = read_all_records(reader);
        let result = total_safe(parse_as_i32s(example_records));
        assert_eq!(result, 516);
    }
}
