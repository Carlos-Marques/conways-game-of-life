use clap::Parser;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

#[derive(Parser)]
struct Opts {
    #[clap(name = "input")]
    input: PathBuf,

    #[clap(long = "badge", short = 'b')]
    badge: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    let input_path = opts.input;
    let file = File::open(&input_path).unwrap();
    let reader = io::BufReader::new(file);

    let priority = match opts.badge {
        false => prioritize_repeated_items(reader),
        true => prioritize_badge(reader),
    };

    println!("{}", priority);
}

fn prioritize_repeated_item<I: Iterator<Item = String>>(rucksacks: I) -> i32 {
    let item = rucksacks
        .map(|rucksack| rucksack.chars().collect::<HashSet<_>>())
        .reduce(|acc, rucksack_chars| acc.intersection(&rucksack_chars).cloned().collect())
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .clone();

    if item.is_lowercase() {
        item as i32 - 'a' as i32 + 1
    } else {
        item as i32 - 'A' as i32 + 27
    }
}

fn prioritize_badge<R: BufRead>(reader: R) -> i32 {
    reader
        .lines()
        .map(|line| line.unwrap())
        .chunks(3)
        .into_iter()
        .map(prioritize_repeated_item)
        .sum()
}

fn prioritize_repeated_items<R: BufRead>(reader: R) -> i32 {
    reader
        .lines()
        .map(|line| line.unwrap())
        .map(|rucksack| {
            let rucksack_len = rucksack.len();
            let (first_half, second_half) = rucksack.split_at(rucksack_len / 2);
            std::iter::once(first_half.to_string()).chain(std::iter::once(second_half.to_string()))
        })
        .map(prioritize_repeated_item)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prioritize_repeated_items() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        let reader = io::BufReader::new(input.as_bytes());
        let score = prioritize_repeated_items(reader);
        assert_eq!(score, 157);
    }

    #[test]
    fn test_prioritize_badge() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        let reader = io::BufReader::new(input.as_bytes());
        let score = prioritize_badge(reader);
        assert_eq!(score, 70);
    }
}
