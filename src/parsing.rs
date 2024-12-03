use std::io::BufRead;

pub fn read_all_records<T>(readable: T) -> Vec<Vec<String>>
where
    T: BufRead,
{
    let mut result: Vec<Vec<String>> = Vec::<Vec<String>>::new();
    for line in readable.lines() {
        let content = line.unwrap();
        let record: Vec<String> = content.split_whitespace().map(String::from).collect();
        result.push(record);
    }
    result
}

pub fn parse_as_ii(input: Vec<Vec<String>>) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();
    for input_record in input {
        assert!(input_record.len() == 2);
        let record: (i32, i32) = (
            input_record[0].parse::<i32>().unwrap(),
            input_record[1].parse::<i32>().unwrap(),
        );
        result.push(record);
    }
    result
}

pub fn parse_as_i32s(input: Vec<Vec<String>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    for input_record in input {
        let mut record: Vec<i32> = Vec::new();
        for e in input_record {
            record.push(e.parse::<i32>().unwrap())
        }
        result.push(record);
    }
    result
}
