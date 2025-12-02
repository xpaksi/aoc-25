use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Debug)]
pub struct Range {
    pub start: i64,
    pub end: i64,
}

impl Range {
    fn parse_range(input: &str) -> Option<Self> {
        let splitted_input: Vec<&str> = input.split('-').collect();
        if splitted_input.len() < 2 {
            return None;
        }

        let start: i64 = splitted_input[0].parse().ok()?;
        let end: i64 = splitted_input[1].parse().ok()?;

        Some(Self {
            start: start,
            end: end,
        })
    }
}

pub fn validate_id(input: String) -> bool {
    let len = input.len();
    let middle = len / 2;

    for candidate in 1..=middle {
        if len % candidate == 0 {
            let candidate_str = &input[0..candidate];
            let repeated = candidate_str.repeat(len / candidate);
            if repeated == input {
                return false;
            }
        }
    }
    return true;
}

pub fn parse_line(input_line: String) -> Vec<Range> {
    let result: Vec<Range> = input_line
        .split(',')
        .map(|x| x.trim())
        .filter_map(|x| Range::parse_range(x))
        .collect();

    return result;
}

pub fn solve(file: File) -> Result<i64, Box<dyn std::error::Error>> {
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let ranges = parse_line(contents);
    let invalid_ids: Vec<i64> = ranges
        .iter()
        .flat_map(|r| r.start..=r.end)
        .filter(|id| {
            let s = id.to_string();
            !s.starts_with('0') && !validate_id(s)
        })
        .collect();

    let sum_invalid_ids: i64 = invalid_ids.iter().sum();
    Ok(sum_invalid_ids)
}
