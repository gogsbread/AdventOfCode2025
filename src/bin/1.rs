use aoc_2024::*;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    write_out(read_in()?);
    Ok(())
}
