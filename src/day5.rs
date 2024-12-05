use crate::parsing::read_lines;

fn read_rules(lines: &Vec<String>) -> Vec<(u32, u32)> {
    let mut result = Vec::new();
    for line in lines {
        if line.contains("|") {
            let rule_elements: Vec<u32> = line
                .split("|")
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .collect();
            assert!(rule_elements.len() == 2);
            println!("found rule {}|{}", rule_elements[0], rule_elements[1]);
            result.push((rule_elements[0], rule_elements[1]));
        }
    }
    result
}

fn read_updates(lines: &Vec<String>) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    for line in lines {
        if line.contains(",") {
            let update: Vec<u32> = line
                .split(",")
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .collect();
            println!("found update {:#?}", update);
            result.push(update);
        }
    }
    result
}

fn valid_update(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    println!("considering update {:#?}:", update);
    for (low, high) in rules {
        let mut seen_high = false;
        for number in update {
            seen_high |= number == high;
            if number == low && seen_high {
                println!("  violated rule {}|{}", low, high);
                return false;
            }
        }
    }
    println!("  Okay.");
    true
}

fn middle_element(update: &Vec<u32>) -> u32 {
    update[(update.len() - 1) / 2]
}

pub fn day5(source: Option<String>) -> i32 {
    let lines = read_lines(source);
    let rules = read_rules(&lines);
    let updates = read_updates(&lines);
    let valid_updates = updates.iter().filter(|&u| valid_update(u, &rules));
    let total: u32 = valid_updates.map(middle_element).sum();
    i32::try_from(total).unwrap()
}

pub fn day5b(_source: Option<String>) -> i32 {
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
