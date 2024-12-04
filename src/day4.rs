use crate::parsing::{read_lines, read_one_string};

fn stride_text(input: &String, start: usize, stride: usize) -> String {
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

fn all_search_lines(input: String) -> Vec<String> {
    // Imagine the grid:
    // 1 2 3 (\n)
    // 4 5 6 (\n)
    // 7 8 9 (\n)
    let w = input.find("\n").unwrap();
    let h = input.len() / (w + 1);
    println!("Input is {} x {}", w, h);
    let mut result = vec![];
    // The horizontal lines: 123 456 789 987 654 321
    result.push(input.clone());
    result.push(input.chars().rev().collect());
    // The vertical lines 147 741 258 852 369 963
    for i in 0..w {
        let line = stride_text(&input, i, w + 1);
        result.push(line.clone());
        result.push(line.chars().rev().collect());
    }
    // The southeast diagonals
    // 159 951 26 62 3 3 7 7 48 84
    for i in 0..=(w + 1) {
        let line = stride_text(&input, i, w + 2);
        result.push(line.clone());
        result.push(line.chars().rev().collect());
    }
    // The southwest diagonals
    // 1 68 86 1 24 9 9 42 357 753
    for i in 0..w {
        let line = stride_text(&input, i, w);
        result.push(line.clone());
        result.push(line.chars().rev().collect());
    }
    result
}

fn count_word_in_lines(input: Vec<String>, target_word: &str) -> i32 {
    let mut result = 0;
    let target_len = target_word.len();
    for line in input.iter() {
        let mut count_in_line = 0;
        if line.len() >= target_len - 1 {
            for i in 0..(line.len() - (target_len - 1)) {
                if &line[i..(i + target_len)] == target_word {
                    count_in_line += 1;
                }
            }
        }
        println!(
            "{}\n{} matches + {} = {}\n",
            line,
            count_in_line,
            result,
            count_in_line + result
        );
        result += count_in_line;
    }
    result
}

fn sliding_windows(input: Vec<String>, w: usize, h: usize) -> Vec<String> {
    vec![]
}

fn find_xs(input: Vec<String>) -> i32 {
    let mut result = 0;
    for vignette in sliding_windows(input, 3, 3) {
        result += 1;
    }
    result
}

fn find_words(input: String) -> i32 {
    let search_lines = all_search_lines(input);
    count_word_in_lines(search_lines, "XMAS")
}

pub fn day4(source: Option<String>) -> i32 {
    let lines = read_one_string(source);
    find_words(lines)
}

pub fn day4b(source: Option<String>) -> i32 {
    let lines = read_lines(source);
    find_xs(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(day4(Some("data/day4_example.txt".to_string())), 18);
    }

    #[test]
    fn test_test() {
        assert_eq!(day4(Some("data/day4_test.txt".to_string())), 2504);
    }
}
