use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

const ROCK_VALUE: i32 = 1;
const PAPER_VALUE: i32 = 2;
const SCISSORS_VALUE: i32 = 3;

const LOSE_VALUE: i32 = 0;
const DRAW_VALUE: i32 = 3;
const WIN_VALUE: i32 = 6;

#[derive(Parser)]
struct Opts {
    #[clap(name = "input")]
    input: PathBuf,

    #[clap(long = "rule", short = 'r')]
    rule: bool,
}

struct Outcomes {
    ax: i32,
    ay: i32,
    az: i32,
    bx: i32,
    by: i32,
    bz: i32,
    cx: i32,
    cy: i32,
    cz: i32,
}

const RULES_1: Outcomes = Outcomes {
    ax: DRAW_VALUE + ROCK_VALUE,
    ay: WIN_VALUE + PAPER_VALUE,
    az: LOSE_VALUE + SCISSORS_VALUE,
    bx: LOSE_VALUE + ROCK_VALUE,
    by: DRAW_VALUE + PAPER_VALUE,
    bz: WIN_VALUE + SCISSORS_VALUE,
    cx: WIN_VALUE + ROCK_VALUE,
    cy: LOSE_VALUE + PAPER_VALUE,
    cz: DRAW_VALUE + SCISSORS_VALUE,
};

const RULES_2: Outcomes = Outcomes {
    ax: LOSE_VALUE + SCISSORS_VALUE,
    ay: DRAW_VALUE + ROCK_VALUE,
    az: WIN_VALUE + PAPER_VALUE,
    bx: LOSE_VALUE + ROCK_VALUE,
    by: DRAW_VALUE + PAPER_VALUE,
    bz: WIN_VALUE + SCISSORS_VALUE,
    cx: LOSE_VALUE + PAPER_VALUE,
    cy: DRAW_VALUE + SCISSORS_VALUE,
    cz: WIN_VALUE + ROCK_VALUE,
};

fn main() {
    let opts: Opts = Opts::parse();

    let input_path = opts.input;
    let file = File::open(&input_path).unwrap();
    let reader = io::BufReader::new(file);

    let outcomes = match opts.rule {
        false => RULES_1,
        true => RULES_2,
    };

    let score = count_score(reader, outcomes);

    println!("{}", score);
}

fn count_score<R: BufRead>(reader: R, outcomes: Outcomes) -> i32 {
    reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let plays: Vec<&str> = line.split(" ").collect();
            let opponent_play = plays[0];
            let my_play = plays[1];

            match (opponent_play, my_play) {
                ("A", "X") => outcomes.ax,
                ("A", "Y") => outcomes.ay,
                ("A", "Z") => outcomes.az,
                ("B", "X") => outcomes.bx,
                ("B", "Y") => outcomes.by,
                ("B", "Z") => outcomes.bz,
                ("C", "X") => outcomes.cx,
                ("C", "Y") => outcomes.cy,
                ("C", "Z") => outcomes.cz,
                _ => panic!("Invalid play"),
            }
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_score() {
        let input = "A Y\nB X\nC Z";
        let reader = io::BufReader::new(input.as_bytes());
        let score = count_score(reader, RULES_1);
        assert_eq!(score, 15);
    }

    #[test]
    fn test_count_score_new_rules() {
        let input = "A Y\nB X\nC Z";
        let reader = io::BufReader::new(input.as_bytes());
        let score = count_score(reader, RULES_2);
        assert_eq!(score, 12);
    }
}
