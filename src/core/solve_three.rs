use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn find_max(elements: Vec<u32>) -> (u32, usize) {
    let mut max = 0;
    let mut max_index = 0;
    let mut index = 0;

    for i in elements {
        if max < i {
            max = i;
            max_index = index;
        }
        index += 1;
    }

    return (max, max_index + 1);
}

pub fn solve_helper(elements: &Vec<u32>) -> Result<u32, Box<dyn std::error::Error>> {
    let (first_part_val, first_part_idx) = find_max(elements[0..=elements.len() - 2].to_vec());

    let (second_part_val, _second_part_idx) =
        find_max(elements[first_part_idx..=elements.len() - 1].to_vec());

    let val: u32 = format!("{}{}", first_part_val, second_part_val).parse()?;
    return Ok(val);
}

pub fn solve(file: File) -> Result<u32, Box<dyn std::error::Error>> {
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let elements: Vec<Vec<u32>> = contents
        .split("\n")
        .map(|line| line.trim().chars().filter_map(|f| f.to_digit(10)).collect())
        .filter(|x: &Vec<u32>| x.len() > 0)
        .collect();

    let result: u32 = elements.iter().filter_map(|x| solve_helper(x).ok()).sum();
    Ok(result)
}
