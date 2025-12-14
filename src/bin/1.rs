use aoc_2024::*;
use std::error;

fn p1<I, S>(input: I) -> usize
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    input
        .into_iter()
        .scan(50, |acc, line| {
            let line = line.as_ref();
            let mut d = line[1..].parse::<i32>().unwrap();
            if line.starts_with('L') {
                d = 100 - d;
            }
            *acc = (*acc + d) % 100;
            Some(*acc)
        })
        .filter(|v| *v == 0)
        .count()
}

fn p2(input: &Vec<String>) -> usize {
    let iter = input.iter().flat_map(|line| {
        let line = line.as_str();
        let d = line[1..].parse::<usize>().unwrap();
        let dir = if line.starts_with('L') { "L1" } else { "R1" };
        std::iter::repeat(dir).take(d).map(|s| s.to_string())
    });
    p1(iter)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_in()?;
    write_out(p1(&input));
    write_out(p2(&input));
    Ok(())
}
