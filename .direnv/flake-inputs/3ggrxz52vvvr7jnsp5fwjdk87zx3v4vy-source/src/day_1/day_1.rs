use clap::Parser;
use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

#[derive(Parser)]
struct Opts {
    #[clap(name = "input")]
    input: PathBuf,

    #[clap(name = "top", default_value = "1")]
    top: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    let input_path = opts.input;
    let file = File::open(&input_path).unwrap();
    let reader = io::BufReader::new(file);
    let max_calories = calculate_max_calories(reader, opts.top).unwrap();
    println!("{}", max_calories);
}

fn calculate_max_calories<R: BufRead>(reader: R, top: usize) -> io::Result<u32> {
    let calories = reader
        .lines()
        .map(|line| line.unwrap())
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter_map(|(is_empty, group)| {
            if is_empty {
                None
            } else {
                Some(
                    group
                        .filter_map(|line| line.parse::<u32>().ok())
                        .sum::<u32>(),
                )
            }
        })
        .sorted();

    let max_calories = calories.into_iter().rev().take(top).sum();

    Ok(max_calories)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_max_calories() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";
        let reader = io::BufReader::new(input.as_bytes());
        let result = calculate_max_calories(reader, 1).unwrap();
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_calculate_max_calories_top_3() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";
        let reader = io::BufReader::new(input.as_bytes());
        let result = calculate_max_calories(reader, 3).unwrap();
        assert_eq!(result, 45000);
    }
}
