use crate::{
    core::solve_one::solve,
    file::open::{open_file, standardize_path},
};

pub fn run(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 2 {
        return Err("Missing input file argument".into());
    }
    let abs_path = standardize_path(args[1].clone().to_string())?;
    let input_file = open_file(abs_path)?;
    let result = solve(input_file)?;
    println!("Day 1 result: {}", result);
    Ok(())
}
