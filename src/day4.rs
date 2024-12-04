use crate::parsing::read_one_string;

fn stride_text(input: &String, start: usize, stride: usize) -> String {
    let mut line_iter = input.chars();
    let mut line = String::new();
    line += &line_iter.nth(start).unwrap().to_string();
    loop {
        match line_iter.nth(stride) {
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
    let w = input.find("\n").unwrap();
    let h = input.len() / (w + 1);
    println!("Input is {} x {}", w, h);
    let mut result = vec![];
    // The horizontal lines
    result.push(input.clone());
    result.push(input.chars().rev().collect());
    // The vertical lines
    for i in 0..w {
        let line = stride_text(&input, i, w);
        result.push(line.clone());
        result.push(line.chars().rev().collect());
    }
    // The southeast diagonals
    for i in 0..w {
        let line = stride_text(&input, i, w + 1);
        result.push(line.clone());
        result.push(line.chars().rev().collect());
    }
    // The southwest diagonals
    for i in 0..w {
        let line = stride_text(&input, i, w - 1);
        result.push(line.clone());
        result.push(line.chars().rev().collect());
    }
    result
}

fn count_xmas_in_lines(input: Vec<String>) -> i32 {
    let mut result = 0;
    for line in input.iter() {
        let mut count_in_line = 0;
        for i in 0..(line.len() - 3) {
            if &line[i..i + 4] == "XMAS" {
                count_in_line += 1;
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

fn find_words(input: String) -> i32 {
    let search_lines = all_search_lines(input);
    count_xmas_in_lines(search_lines)
}

pub fn day4(source: Option<String>) -> i32 {
    let lines = read_one_string(source);
    find_words(lines)
}

pub fn day4b(source: Option<String>) -> i32 {
    let lines = read_one_string(source);
    find_words(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(day4(Some("data/day4_example.txt".to_string())), 18);
    }
}
