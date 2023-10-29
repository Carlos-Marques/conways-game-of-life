use clap::Parser;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Parser)]
struct Opts {
    #[clap(name = "board")]
    board: PathBuf,

    #[clap(name = "kernel")]
    kernel: PathBuf,

    #[clap(short, long, default_value = "500")]
    sleep_millis: u64,
}

fn main() {
    let opts: Opts = Opts::parse();

    let board_reader = get_file_reader(opts.board);
    let ((board_height, board_width), mut board) = parse_matrix(board_reader);

    let kernel_reader = get_file_reader(opts.kernel);
    let ((kernel_height, kernel_width), kernel) = parse_matrix(kernel_reader);

    loop {
        let neighbors = apply_kernel(
            &board,
            board_width,
            board_height,
            &kernel,
            kernel_width,
            kernel_height,
        );

        for index in 0..board.len() {
            board[index] = match (board[index], neighbors[index]) {
                (1, 2) | (1, 3) | (0, 3) => 1,
                _ => 0,
            };
        }

        print!("\x1B[2J\x1B[1;1H");
        for row in 0..board_height {
            for column in 0..board_width {
                print!("{} ", board[row * board_width + column]);
            }
            println!();
        }

        std::thread::sleep(std::time::Duration::from_millis(opts.sleep_millis));
    }
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

fn is_within_boundaries(
    input_row: isize,
    input_column: isize,
    input_height: isize,
    input_width: isize,
) -> bool {
    input_row >= 0
        && input_row < input_height as isize
        && input_column >= 0
        && input_column < input_width as isize
}

fn get_input_location(
    matrix_location: usize,
    kernel_location: usize,
    kernel_offset: usize,
) -> isize {
    matrix_location as isize + kernel_location as isize - kernel_offset as isize
}

fn get_input_position(
    row: usize,
    column: usize,
    kernel_row: usize,
    kernel_column: usize,
    kernel_offset_height: usize,
    kernel_offset_width: usize,
) -> (isize, isize) {
    let input_row = get_input_location(row, kernel_row, kernel_offset_height);
    let input_column = get_input_location(column, kernel_column, kernel_offset_width);

    (input_row, input_column)
}

fn get_1d_index(row: usize, width: usize, column: usize) -> usize {
    row * width + column
}

fn apply_kernel(
    input: &[i32],
    input_width: usize,
    input_height: usize,
    kernel: &[i32],
    kernel_width: usize,
    kernel_height: usize,
) -> Vec<i32> {
    let kernel_offset_width = kernel_width / 2;
    let kernel_offset_height = kernel_height / 2;

    (0..input_height)
        .flat_map(|row| {
            (0..input_width).map(move |column| {
                (0..kernel_height)
                    .flat_map(|kernel_row| {
                        (0..kernel_width).filter_map(move |kernel_column| {
                            let (input_row, input_column) = get_input_position(
                                row,
                                column,
                                kernel_row,
                                kernel_column,
                                kernel_offset_height,
                                kernel_offset_width,
                            );

                            if is_within_boundaries(
                                input_row,
                                input_column,
                                input_height as isize,
                                input_width as isize,
                            ) {
                                let input_index = get_1d_index(
                                    input_row as usize,
                                    input_width,
                                    input_column as usize,
                                );
                                let kernel_index =
                                    get_1d_index(kernel_row, kernel_width, kernel_column);

                                Some(input[input_index] * kernel[kernel_index])
                            } else {
                                None
                            }
                        })
                    })
                    .sum::<i32>()
            })
        })
        .collect()
}
