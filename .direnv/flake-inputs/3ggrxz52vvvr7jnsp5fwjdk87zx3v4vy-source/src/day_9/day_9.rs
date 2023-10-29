use clap::Parser;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Parser)]
struct Opts {
    #[clap(name = "input")]
    input: PathBuf,

    #[clap(short, long, default_value = "2")]
    knots: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    let input_path = opts.input;
    let file = File::open(&input_path).unwrap();
    let reader = BufReader::new(file);

    let result = count_unique_positions(reader, opts.knots);

    println!("{}", result)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_in_direction(&mut self, knot_move: &Direction) {
        match knot_move {
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        }
    }
}

impl std::ops::Sub for &Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
pub enum ParseMoveError {
    InvalidDirection,
    InvalidSteps,
}

impl std::fmt::Display for ParseMoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseMoveError::InvalidDirection => write!(f, "Invalid direction"),
            ParseMoveError::InvalidSteps => write!(f, "Invalid steps"),
        }
    }
}

impl std::error::Error for ParseMoveError {}

pub struct Directions(Vec<Direction>);
impl FromStr for Directions {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let mut directions = Vec::new();
        let steps = parts[1]
            .parse::<i32>()
            .map_err(|_| ParseMoveError::InvalidSteps)?;

        for _ in 0..steps {
            let direction = match parts[0] {
                "R" => Ok(Direction::Right),
                "L" => Ok(Direction::Left),
                "U" => Ok(Direction::Up),
                "D" => Ok(Direction::Down),
                _ => Err(ParseMoveError::InvalidDirection),
            }?;
            directions.push(direction);
        }

        Ok(Directions(directions))
    }
}

fn count_unique_positions<R: BufRead>(reader: R, num_points: usize) -> usize {
    reader
        .lines()
        .map(|line| line.unwrap().parse::<Directions>().unwrap().0)
        .flatten()
        .scan(
            vec![Point { x: 0, y: 0 }; num_points],
            |points, direction: Direction| {
                points[0].move_in_direction(&direction);

                for i in 1..num_points {
                    let difference_between_knots = &points[i - 1] - &points[i];
                    match (
                        difference_between_knots.x.abs(),
                        difference_between_knots.y.abs(),
                    ) {
                        (2, 1) | (1, 2) | _
                            if difference_between_knots.x.abs() > 1
                                || difference_between_knots.y.abs() > 1 =>
                        {
                            points[i].x += difference_between_knots.x.signum();
                            points[i].y += difference_between_knots.y.signum();
                        }
                        _ => {}
                    }
                }

                Some(points.last().unwrap().clone())
            },
        )
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_count_unique_positions_2_knots() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        let reader = BufReader::new(input.as_bytes());

        let tail_count = count_unique_positions(reader, 2);

        assert_eq!(tail_count, 13);
    }

    #[test]
    fn test_count_unique_positions_10_knots() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        let reader = BufReader::new(input.as_bytes());
        let tail_count = count_unique_positions(reader, 10);
        assert_eq!(tail_count, 1);

        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        let reader = BufReader::new(input.as_bytes());
        let tail_count = count_unique_positions(reader, 10);
        assert_eq!(tail_count, 36);
    }
}
