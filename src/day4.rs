use crate::parsing::read_lines;

/** Selects a range of a string, but truncates the range requested to avoid
 * errors.
 */
fn wide_range(input: String, start: i32, end: i32) -> String {
    let ustart: usize = if start < 0 { 0 } else { usize::try_from(start).unwrap() };
    let uend: usize = if end < 0 { 0 } else { usize::try_from(end).unwrap() };
    
    input[usize::min(ustart, input.len())
          ..usize::min(uend, input.len())].to_string()
}

fn all_search_lines(input: Vec<String>) -> Vec<String> {
    let h = input.len();
    let w = input[0].len();
    let mut result = vec!();
    // The horizontal lines
    result.append(&mut input.clone());
    // The vertical lines
    for i in 0..w {
        result.push(input.iter().map(|l| l.chars().nth(i).unwrap()).collect());
    }
    // The southeast diagonals
    for i in 0..h {
        ...
    }
    for i in 1..w {
        ...
    }
    // The southwest diagonals
    for i in 1..w {
        ...
    }
    for i in 0..h {
        ...
    }
    result
}

fn count_xmas_in_lines(input: Vec<String>) -> i32 {
    let mut result = 0;
    for line in input.iter() {
        for i in 0..(line.len() - 3) {
            if &line[i..i+4] == "XMAS" {
                result += 1;
            }
        }
    }
    result
}

fn find_words(input: Vec<String>) -> i32{
    let search_lines = all_search_lines(input);
    count_xmas_in_lines(search_lines)
}


pub fn day4(source: Option<String>) -> i32 {
    let lines = read_lines(source);
    find_words(lines)
}

pub fn day4b(source: Option<String>) -> i32 {
    let lines = read_lines(source);
    find_words(lines)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(day4(Some("data/day4_example.txt".to_string())), 18);
    }
/*
    #[test]
    fn test_test() {
        assert_eq!(day4(Some("data/day4_test.txt".to_string())), 165225049);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(day4b(Some("data/day4_example_b.txt".to_string())), 48);
    }
    
    #[test]
    fn test_test_b() {
        assert_eq!(day4b(Some("data/day4_test.txt".to_string())), 108830766);
    }
*/
}