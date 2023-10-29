use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser)]
struct Opts {
    #[clap(name = "input")]
    input: PathBuf,

    #[clap(long = "order", short = 'o')]
    order: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    let input_path = opts.input;
    let mut file = File::open(&input_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let split_index = contents.find("\n\n").unwrap();
    let (starting_positions, moves_str) = contents.split_at(split_index);

    let stacks = parse_stacks(starting_positions);

    let moves = parse_moves(moves_str);

    let stacks = reorganize_crates(stacks, moves, opts.order);
    let top_crates = get_top_crates(stacks);

    println!("{}", top_crates);
}

fn parse_stacks(starting_positions: &str) -> Vec<Vec<String>> {
    let last_line = starting_positions.lines().last().unwrap();
    let stacks_index: Vec<&str> = last_line.split_whitespace().collect();

    starting_positions
        .lines()
        .rev()
        .skip(1)
        .flat_map(|line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != ' ' && *c != '[' && *c != ']')
                .map(|(index, c)| {
                    let skipped_indices = index / 4;
                    let adjusted_index = index - skipped_indices;
                    let stack_index = adjusted_index / 3;
                    (stack_index, c.to_string())
                })
        })
        .fold(
            vec![Vec::new(); stacks_index.len()],
            |mut acc, (stack_index, c)| {
                acc[stack_index].push(c);
                acc
            },
        )
}

#[derive(PartialEq, Debug)]
struct Move {
    number: usize,
    from: usize,
    to: usize,
}

fn parse_moves(input: &str) -> Vec<Move> {
    let mut moves = Vec::new();

    for line in input.lines() {
        if let Some((number, from, to)) = parse_move(line) {
            moves.push(Move { number, from, to });
        }
    }

    moves
}

fn parse_move(line: &str) -> Option<(usize, usize, usize)> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() == 6 && tokens[0] == "move" && tokens[2] == "from" && tokens[4] == "to" {
        let number = tokens[1].parse::<usize>().ok()?;
        let from = tokens[3].parse::<usize>().ok()?;
        let to = tokens[5].parse::<usize>().ok()?;
        Some((number, from, to))
    } else {
        None
    }
}

fn reorganize_crates(
    mut stacks: Vec<Vec<String>>,
    moves: Vec<Move>,
    order: bool,
) -> Vec<Vec<String>> {
    for each_move in moves {
        let mut carried_crates = Vec::new();

        let stack_from = &mut stacks[each_move.from - 1];
        for _ in 0..each_move.number {
            carried_crates.push(stack_from.pop());
        }

        match order {
            false => {}
            true => carried_crates.reverse(),
        };

        let stack_to = &mut stacks[each_move.to - 1];
        for stack_crate in carried_crates.into_iter() {
            if let Some(stack_crate) = stack_crate {
                stack_to.push(stack_crate);
            }
        }
    }

    return stacks;
}

fn get_top_crates(mut stacks: Vec<Vec<String>>) -> String {
    stacks
        .iter_mut()
        .filter_map(|stack| stack.pop())
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_crates() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";

        let split_index = input.find("\n\n").unwrap();
        let (starting_positions, moves_str) = input.split_at(split_index);

        let stacks = parse_stacks(starting_positions);
        assert_eq!(stacks, vec![vec!["Z", "N"], vec!["M", "C", "D"], vec!["P"]]);

        let moves = parse_moves(moves_str);
        assert_eq!(
            moves,
            vec![
                Move {
                    number: 1,
                    from: 2,
                    to: 1
                },
                Move {
                    number: 3,
                    from: 1,
                    to: 3
                },
                Move {
                    number: 2,
                    from: 2,
                    to: 1
                },
                Move {
                    number: 1,
                    from: 1,
                    to: 2
                }
            ]
        );

        let stacks = reorganize_crates(stacks, moves, false);
        let top_crates = get_top_crates(stacks);
        assert_eq!(top_crates, "CMZ");
    }

    #[test]
    fn test_top_crates_same_order() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";

        let split_index = input.find("\n\n").unwrap();
        let (starting_positions, moves_str) = input.split_at(split_index);

        let stacks = parse_stacks(starting_positions);
        assert_eq!(stacks, vec![vec!["Z", "N"], vec!["M", "C", "D"], vec!["P"]]);

        let moves = parse_moves(moves_str);
        assert_eq!(
            moves,
            vec![
                Move {
                    number: 1,
                    from: 2,
                    to: 1
                },
                Move {
                    number: 3,
                    from: 1,
                    to: 3
                },
                Move {
                    number: 2,
                    from: 2,
                    to: 1
                },
                Move {
                    number: 1,
                    from: 1,
                    to: 2
                }
            ]
        );

        let stacks = reorganize_crates(stacks, moves, true);
        let top_crates = get_top_crates(stacks);
        assert_eq!(top_crates, "MCD");
    }
}
