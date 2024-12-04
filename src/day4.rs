use crate::parsing::read_one_string;

fn all_search_lines(input: String) -> Vec<String> {
    let w = input.find("\n").unwrap();
    let h = input.len() / (w + 1);
    println!("Input is {} x {}", w, h);
    let mut result = vec!();
    // The horizontal lines
    result.push(input);
    // The vertical lines

    // The southeast diagonals

    // The southwest diagonals

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

fn find_words(input: String) -> i32{
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