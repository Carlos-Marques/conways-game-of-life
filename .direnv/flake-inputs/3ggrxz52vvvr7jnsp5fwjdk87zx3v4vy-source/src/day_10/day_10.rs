use clap::Parser;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::FromStr;

const NOOP_N_CYCLES: usize = 1;
const ADDX_N_CYCLES: usize = 2;
const CHECK_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

#[derive(Parser)]
struct Opts {
    #[clap(name = "input")]
    input: PathBuf,

    #[clap(long = "crt", short = 'c')]
    crt: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    let input_path = opts.input;
    let file = File::open(&input_path).unwrap();
    let reader = BufReader::new(file);

    let register_values = get_register_values(reader);

    let result: String = match opts.crt {
        true => register_values
            .map(|(register, cycle)| {
                match (register - 1..=register + 1).contains(&(((cycle as i32) - 1) % 40)) {
                    true if (cycle as i32 % 40) == 0 => "#\n",
                    true => "#",
                    false if (cycle as i32 % 40) == 0 => ".\n",
                    false => ".",
                }
            })
            .collect::<String>(),
        false => register_values
            .filter(|(_, cycle)| CHECK_CYCLES.contains(cycle))
            .map(|(register, cycle)| register * (cycle as i32))
            .sum::<i32>()
            .to_string(),
    };

    println!("{}", result)
}

enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Debug)]
enum ParseInstructionError {
    InvalidInstruction,
    ParseIntError(ParseIntError),
}

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseInstructionError::InvalidInstruction => write!(f, "Invalid instruction"),
            ParseInstructionError::ParseIntError(e) => e.fmt(f),
        }
    }
}

impl From<ParseIntError> for ParseInstructionError {
    fn from(err: ParseIntError) -> ParseInstructionError {
        ParseInstructionError::ParseIntError(err)
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts[0] {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::Addx(parts[1].parse()?)),
            _ => Err(ParseInstructionError::InvalidInstruction),
        }
    }
}

fn get_register_values<R: BufRead>(reader: R) -> impl Iterator<Item = (i32, usize)> {
    reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .scan(
            (1, 0),
            |(register, cycle), instruction: Instruction| match instruction {
                Instruction::Noop => {
                    *cycle += NOOP_N_CYCLES;
                    let register_values = vec![(*register, *cycle)];
                    Some(register_values)
                }
                Instruction::Addx(value) => {
                    let register_values = (1..=ADDX_N_CYCLES)
                        .map(|_| {
                            *cycle += 1;
                            (*register, *cycle)
                        })
                        .collect::<Vec<_>>();

                    *register += value;

                    Some(register_values)
                }
            },
        )
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_signal_strengths() {
        let input = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";
        let reader = BufReader::new(input.as_bytes());

        let result: i32 = get_register_values(reader)
            .filter(|(_, cycle)| CHECK_CYCLES.contains(cycle))
            .map(|(register, cycle)| register * (cycle as i32))
            .sum();

        assert_eq!(result, 13140);
    }
}
