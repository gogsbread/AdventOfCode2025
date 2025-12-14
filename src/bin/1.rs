use aoc_2024::*;
use std::error;

fn p1(input: &Vec<String>) -> usize {
    let cum_v = input.iter().scan(50, |acc, line| {
        let mut d = line[1..].parse::<i32>().unwrap();
        if line.starts_with('L') {
            d = 100 - d;
        }
        *acc = (*acc + d) % 100;
        Some(*acc)
    });
    cum_v.filter(|v| *v == 0).count()
}

fn p2(input: &Vec<String>) -> usize {
    let input = input
        .iter()
        .flat_map(|line| {
            let d = line[1..].parse::<i32>().unwrap();
            if line.starts_with('L') {
                (0..d).map(|_| "L1".to_string()).collect::<Vec<_>>()
            } else {
                (0..d).map(|_| "R1".to_string()).collect::<Vec<_>>()
            }
        })
        .collect::<Vec<String>>();
    p1(&input)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_in()?;
    write_out(p1(&input));
    write_out(p2(&input));
    Ok(())
}
