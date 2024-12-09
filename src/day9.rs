use crate::parsing::read_lines;

type Scalar = i32;

type Id = Scalar;
type Length = Scalar;

#[derive (PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct RleItem {
    id: Option<Id>,
    length: Length,
}
type RleList = Vec<RleItem>;

fn str_to_rle(source: &String) -> RleList {
    let mut result = RleList::new();
    let mut next_id: Id = 0;
    let mut chars = source.chars();
    loop {
        let file_len_opt = chars.next();
        if let Some(file_len_char) = file_len_opt {
            result.push(RleItem{id:Some(next_id),
                                length: file_len_char.to_string().parse().unwrap()});
            next_id += 1;
        } else {
            break;
        }
        let file_len_opt = chars.next();
        if let Some(file_len_char) = file_len_opt {
            result.push(RleItem{id:None,
                                length: file_len_char.to_string().parse().unwrap()});
        } else {
            break;
        }
    }
    result
}

fn rle_to_map_str(rle: &RleList) -> String {
    let mut result = String::new();
    for RleItem{id, length} in rle {
        match id {
            Some(i) => { for _ in 0..*length { result += &(i % 10).to_string(); } }
            None => { for _ in 0..*length { result += &".".to_string(); } }
        }
    }
    result
}


fn is_compact(rle: &RleList) -> bool {
    for i in 0..rle.len()-1 {
        let RleItem{id, length} = rle[i];
        if id.is_none() && length > 0 { return false; }
    }
    true
}

fn index_of_first_empty_item(rle: &RleList) -> usize {
    rle.iter().position(|item| item.id.is_none()).unwrap()
}

fn index_of_last_nonempty_item(rle: &RleList) -> usize {
    let reverse_pos = rle.iter().rev().position(|item| !item.id.is_none()).unwrap();
    rle.len() - reverse_pos - 1
}

fn write_block(rle: &mut RleList, item: &RleItem) {
    let id = item.id;
    let mut remaining_length = item.length;
    let mut insert_index = index_of_first_empty_item(rle);
    while rle[insert_index].length < remaining_length {
        rle[insert_index].id = id;
        remaining_length -= rle[insert_index].length;
        insert_index = index_of_first_empty_item(rle);
    }
    if remaining_length == 0 { return; }  // Avoid inserting zero-length blocks.
    rle[insert_index].length -= remaining_length;
    rle.insert(insert_index, RleItem{id:id, length:remaining_length});
}

fn move_last_nonempty_block(rle: &mut RleList) {
    let index = index_of_last_nonempty_item(rle);
    println!("Last nonempty index is {}", index);
    let item: RleItem = {
        let mut_item: &mut RleItem = rle.get_mut(index).unwrap();
        mut_item.length = 0;
        mut_item.clone()
    };  // Drop mutability
    write_block(rle, &item);
}

pub fn day9(source: Option<String>) -> i64 {
    let lines = read_lines(source);
    let mut rle = str_to_rle(&lines[0]);
    while !is_compact(&rle) {
        println!("{}", rle_to_map_str(&rle));
        move_last_nonempty_block(&mut rle);
    }
    println!("{}", rle_to_map_str(&rle));
    0
}

pub fn day9b(source: Option<String>) -> i64 { 
    day9(source)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(day9(Some("data/day9_example.txt".to_string())), 14);
    }

    #[test]
    fn test_test() {
        assert_eq!(day9(Some("inputs/day9_test.txt".to_string())), 344);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(day9b(Some("data/day9_example.txt".to_string())), 34);
    }

    #[test]
    fn test_test_b() {
        assert_eq!(day9b(Some("inputs/day9_test.txt".to_string())), 1182);
    }
}