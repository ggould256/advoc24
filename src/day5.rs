use crate::parsing::{read_lines};

fn read_rules(lines: &Vec<String>) -> Vec<(u32, u32)> {
    let mut result = Vec::new();
    for line in lines {
        if line.contains("|") {
            let rule_elements: Vec<u32> = line.split("|").map(str::parse::<u32>).map(Result::unwrap).collect();
            assert!(rule_elements.len() == 2);
            result.push((rule_elements[0], rule_elements[1]));
        }
    }
    result
}

fn read_updates(lines: &Vec<String>) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    for line in lines {
        if line.contains(".") {
            let update: Vec<u32> = line.split(",").map(str::parse::<u32>).map(Result::unwrap).collect();
            result.push((update));
        }
    }
    result
}

fn valid_update(update: Vec<u32>, rules: Vec<(u32, u32)>) -> bool {

}

pub fn day5(source: Option<String>) -> i32 {
    let lines = read_lines(source);
    let rules = read_rules(lines);
    let updates = read_updates(lines);
    let valid_updates = filter_updates(updates, rules);
    valid_updates.map(middle_element).sum()
}

pub fn day5b(source: Option<String>) -> i32 {
    let lines = read_lines(source);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(day5(Some("data/day5_example.txt".to_string())), 143);
    }
}