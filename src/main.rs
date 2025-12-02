use std::env;
use std::panic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Invalid args, please provide filename");
    }
    let abs_path = aoc_25::file::open::standardize_path(args[1].clone().to_string())?;
    let input_file = aoc_25::file::open::open_file(abs_path)?;
    let result = aoc_25::core::solve::solve(input_file)?;
    println!("{}", result);
    Ok(())
}
