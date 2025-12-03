use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().into_iter();
    aoc_25::day::three::run(args.collect())?;
    Ok(())
}
