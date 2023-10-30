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

pub fn run(
    iterations: usize,
    board: &mut [i32],
    board_width: usize,
    board_height: usize,
    kernel: &[i32],
    kernel_width: usize,
    kernel_height: usize,
) {
    for _ in 0..iterations {
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
    }
}
