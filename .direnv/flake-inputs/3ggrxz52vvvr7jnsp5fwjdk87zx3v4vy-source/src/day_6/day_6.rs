use clap::Parser;
use itertools::Itertools;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser)]
struct Opts {
    #[clap(name = "input")]
    input: PathBuf,

    #[clap(name = "window_size", default_value = "4")]
    window_size: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    let input_path = opts.input;
    let mut file = File::open(&input_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let end_position = find_start_of_packet(&contents, opts.window_size, 1);

    println!("{}", end_position);
}

fn find_start_of_packet(input: &str, window_size: usize, step: usize) -> usize {
    let (index, _) = input
        .chars()
        .collect::<Vec<_>>()
        .windows(window_size)
        .step_by(step)
        .find_position(|window| {
            window
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len()
                == window_size
        })
        .unwrap();

    index + window_size
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_start_of_packet() {
        let examples = vec![
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (input, answer) in examples {
            assert_eq!(find_start_of_packet(input, 4, 1), answer);
        }
    }

    #[test]
    fn test_find_start_of_message() {
        let examples = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (input, answer) in examples {
            assert_eq!(find_start_of_packet(input, 14, 1), answer);
        }
    }
}
