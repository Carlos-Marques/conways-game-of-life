use clap::Parser;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Parser)]
struct Opts {
    #[clap(name = "input")]
    input: PathBuf,

    #[clap(long = "best_start", short = 'b')]
    best_start: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    let input_path = opts.input;
    let file = File::open(&input_path).unwrap();
    let reader = BufReader::new(file);
    let locations = parse_locations(reader);
    let n_steps = steps_to_best_signal(locations, opts.best_start);

    println!("{}", n_steps);
}

#[derive(Debug)]
struct Location {
    height: i32,
    steps: Option<usize>,
}

fn parse_locations<R: BufRead>(reader: R) -> Vec<Vec<Location>> {
    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|height| Location {
                    height: height as i32,
                    steps: None,
                })
                .collect()
        })
        .collect()
}

fn find_location(locations: &Vec<Vec<Location>>, char_to_find: char) -> (usize, usize) {
    locations
        .iter()
        .enumerate()
        .flat_map(|(row, locations_row)| {
            locations_row
                .iter()
                .enumerate()
                .map(move |(column, _)| (row, column))
        })
        .find(|(row, column)| locations[*row][*column].height == char_to_find as i32)
        .unwrap()
}

fn queue_search(
    mut locations: Vec<Vec<Location>>,
    starting_position: (usize, usize),
    search_condition: impl Fn((i32, i32)) -> bool,
) -> Vec<Vec<Location>> {
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    queue.push_back((starting_position.0, starting_position.1, 0));

    while let Some((row, column, steps)) = queue.pop_front() {
        if row < locations.len() && column < locations[0].len() {
            let location = &mut locations[row][column];

            if location.steps.is_none() || location.steps.unwrap() > steps {
                location.steps = Some(steps);
                let current_height = location.height;

                if row > 0 {
                    let next_location = &locations[row - 1][column];
                    if search_condition((next_location.height, current_height))
                        && (next_location.steps.is_none()
                            || next_location.steps.unwrap() > steps + 1)
                    {
                        queue.push_back((row - 1, column, steps + 1));
                    }
                }

                if column > 0 {
                    let next_location = &locations[row][column - 1];
                    if search_condition((next_location.height, current_height))
                        && (next_location.steps.is_none()
                            || next_location.steps.unwrap() > steps + 1)
                    {
                        queue.push_back((row, column - 1, steps + 1));
                    }
                }

                if row < locations.len() - 1 {
                    let next_location = &locations[row + 1][column];
                    if search_condition((next_location.height, current_height))
                        && (next_location.steps.is_none()
                            || next_location.steps.unwrap() > steps + 1)
                    {
                        queue.push_back((row + 1, column, steps + 1));
                    }
                }

                if column < locations[0].len() - 1 {
                    let next_location = &locations[row][column + 1];
                    if search_condition((next_location.height, current_height))
                        && (next_location.steps.is_none()
                            || next_location.steps.unwrap() > steps + 1)
                    {
                        queue.push_back((row, column + 1, steps + 1));
                    }
                }
            }
        }
    }

    locations
}

fn steps_to_best_signal(mut locations: Vec<Vec<Location>>, best_start: bool) -> usize {
    let start_location = find_location(&locations, 'S');
    locations[start_location.0][start_location.1].height = 'a' as i32;

    let end_location = find_location(&locations, 'E');
    locations[end_location.0][end_location.1].height = 'z' as i32;

    match best_start {
        true => {
            locations = queue_search(locations, end_location, |(next_height, current_height)| {
                (next_height - current_height) >= -1
            });
            locations
                .iter()
                .flatten()
                .filter(|location| location.height == 'a' as i32)
                .min_by_key(|location| location.steps.unwrap_or(std::usize::MAX))
                .unwrap()
                .steps
                .unwrap()
        }
        false => {
            locations = queue_search(
                locations,
                start_location,
                |(next_height, current_height)| (next_height - current_height) <= 1,
            );
            locations[end_location.0][end_location.1].steps.unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steps_to_best_signal() {
        let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";
        let reader = BufReader::new(input.as_bytes());

        let locations = parse_locations(reader);
        let n_steps = steps_to_best_signal(locations, false);

        assert_eq!(n_steps, 31);
    }

    #[test]
    fn test_steps_to_best_start() {
        let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";
        let reader = BufReader::new(input.as_bytes());

        let locations = parse_locations(reader);
        let n_steps = steps_to_best_signal(locations, true);

        assert_eq!(n_steps, 29);
    }
}
