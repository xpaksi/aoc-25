use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn find_max(elements: &Vec<u32>, start_index: usize, end_index: usize) -> (u32, usize) {
    let (max, max_index) =
        elements
            .iter()
            .enumerate()
            .fold((0, 0), |(max, max_index), (index, &val)| {
                if (max < val) && index >= start_index && index <= end_index {
                    (val, index)
                } else {
                    (max, max_index)
                }
            });
    return (max, max_index + 1);
}

pub fn solve_helper(elements: &Vec<u32>) -> Result<u64, Box<dyn std::error::Error>> {
    let (first_part_val, first_part_idx) = find_max(elements, 0, elements.len() - 2);

    let (second_part_val, _second_part_idx) =
        find_max(elements, first_part_idx, elements.len() - 1);

    let val: u64 = format!("{}{}", first_part_val, second_part_val).parse()?;
    return Ok(val);
}

pub fn solve_helper_two(elements: &Vec<u32>) -> Result<u64, Box<dyn std::error::Error>> {
    let mut result_vec: Vec<u32> = Vec::with_capacity(12);
    let mut start_idx: usize = 0;

    for i in (0..12).rev() {
        let (val, val_idx) = find_max(elements, start_idx, elements.len() - (i + 1));
        start_idx = val_idx;
        result_vec.push(val);
    }

    let result_str: Vec<String> = result_vec.iter().map(|x| x.to_string()).collect();
    let result: u64 = result_str.join("").parse()?;
    Ok(result)
}

pub fn solve(file: File) -> Result<u64, Box<dyn std::error::Error>> {
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let elements: Vec<Vec<u32>> = contents
        .split("\n")
        .map(|line| line.trim().chars().filter_map(|f| f.to_digit(10)).collect())
        .filter(|x: &Vec<u32>| x.len() > 0)
        .collect();

    let result: u64 = elements
        .iter()
        .filter_map(|x| solve_helper_two(x).ok())
        .sum();
    Ok(result)
}
