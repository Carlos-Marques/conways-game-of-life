#![feature(portable_simd)]

use clap::Parser;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::Instant;

#[allow(dead_code)]
mod conv;
#[allow(dead_code)]
mod sumcounts;
#[allow(dead_code)]
mod vectorized;

const ITERATIONS: usize = 1_000_000;

#[derive(Parser)]
struct Opts {
    #[clap(name = "board")]
    board: PathBuf,

    #[clap(name = "kernel")]
    kernel: PathBuf,

    #[clap(short, long, default_value = "500")]
    sleep_millis: u64,
}

#[inline(never)]
fn run_board(
    board: &mut [i32],
    board_width: usize,
    board_height: usize,
    kernel: &[i32],
    kernel_width: usize,
    kernel_height: usize,
) {
    // use conv as strategy;
    // use sumcounts as strategy;
    use vectorized as strategy;

    strategy::run(
        ITERATIONS,
        board,
        board_width,
        board_height,
        kernel,
        kernel_width,
        kernel_height,
    );
}

fn main() {
    let opts: Opts = Opts::parse();

    let board_reader = get_file_reader(opts.board);
    let ((board_height, board_width), mut board) = parse_matrix(board_reader);

    let kernel_reader = get_file_reader(opts.kernel);
    let ((kernel_height, kernel_width), kernel) = parse_matrix(kernel_reader);

    println!("Done reading files");

    let start = Instant::now();

    run_board(
        &mut board,
        board_width,
        board_height,
        &kernel,
        kernel_width,
        kernel_height,
    );

    let duration = start.elapsed();

    print!("\x1B[2J\x1B[1;1H");
    for row in 0..board_height {
        for column in 0..board_width {
            print!("{} ", board[row * board_width + column]);
        }
        println!();
    }
    println!("Time elapsed for {ITERATIONS} iters is: {:?}", duration);
}

fn get_file_reader(path: PathBuf) -> BufReader<File> {
    let file = File::open(&path).unwrap();
    BufReader::new(file)
}

fn parse_matrix<R: BufRead>(reader: R) -> ((usize, usize), Vec<i32>) {
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let height = lines.len();
    let width = lines.get(0).map_or(0, |line| line.len());

    let matrix: Vec<i32> = lines
        .into_iter()
        .flat_map(|line| {
            let chars: Vec<char> = line.chars().collect();
            chars.into_iter().map(|c| c.to_digit(10).unwrap() as i32)
        })
        .collect();

    ((height, width), matrix)
}
