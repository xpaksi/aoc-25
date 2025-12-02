use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub enum Direction {
    Left,
    Right,
}
pub struct LineElement {
    pub direction: Direction,
    pub value: i32,
}

pub fn parse_line(input_line: String) -> Option<LineElement> {
    if input_line.is_empty() {
        return None;
    }

    let direction = match input_line.chars().next() {
        Some('L') => Direction::Left,
        Some('R') => Direction::Right,
        _ => return None,
    };

    let value: i32 = input_line[1..].trim().parse().ok()?;
    Some(LineElement { direction, value })
}

pub fn solve_helper(element: LineElement, arrow: i32) -> i32 {
    match element.direction {
        Direction::Left => {
            return (arrow - element.value) % 100;
        }
        Direction::Right => {
            return (arrow + element.value) % 100;
        }
    }
}

pub fn solve(file: File) -> Result<i32, Box<dyn std::error::Error>> {
    let mut start_arrow = 50;
    let mut result = 0;
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        let line = line?;
        let element = parse_line(line);
        match element {
            Some(e) => {
                start_arrow = solve_helper(e, start_arrow);
            }
            None => continue,
        }
        if start_arrow == 0 {
            result = result + 1;
        }
    }

    Ok(result)
}
