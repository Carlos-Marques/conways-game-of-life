use clap::Parser;
use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

#[derive(Parser)]
struct Opts {
    #[clap(name = "input")]
    input: PathBuf,

    #[clap(long = "overlaps", short = 'o')]
    overlaps: bool,
}

fn check_contains(((a1, a2), (b1, b2)): (&(i32, i32), &(i32, i32))) -> bool {
    (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2)
}

fn check_overlaps(((a1, a2), (b1, b2)): (&(i32, i32), &(i32, i32))) -> bool {
    !(a2 < b1 || b2 < a1)
}

fn main() {
    let opts: Opts = Opts::parse();

    let input_path = opts.input;
    let file = File::open(&input_path).unwrap();
    let reader = io::BufReader::new(file);
    let condition = match opts.overlaps {
        false => check_contains,
        true => check_overlaps,
    };
    let number_pair_contains = count_pair_check(reader, condition);

    println!("{}", number_pair_contains);
}

fn count_pair_check<R: BufRead, F>(reader: R, condition: F) -> usize
where
    F: Fn((&(i32, i32), &(i32, i32))) -> bool,
{
    reader
        .lines()
        .map(|line| line.unwrap())
        .map(|assignment_pairs| {
            assignment_pairs
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .map(|split_pairs| {
            split_pairs
                .iter()
                .map(|pair| {
                    let bounds: Vec<i32> =
                        pair.split("-").map(|s| s.parse::<i32>().unwrap()).collect();
                    (bounds[0], bounds[1])
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .filter(move |pairs| {
            pairs
                .iter()
                .tuple_combinations()
                .any(|pair| condition(pair))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_pair_contains() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        let reader = io::BufReader::new(input.as_bytes());
        let number_pair_contains = count_pair_check(reader, check_contains);
        assert_eq!(number_pair_contains, 2);
    }

    #[test]
    fn test_count_pair_overlaps() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        let reader = io::BufReader::new(input.as_bytes());
        let number_pair_contains = count_pair_check(reader, check_overlaps);
        assert_eq!(number_pair_contains, 4);
    }
}
