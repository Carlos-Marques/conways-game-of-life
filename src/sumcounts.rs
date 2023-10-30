fn compute_counts(counts: &mut [i32], board: &[i32], width: usize, height: usize) {
    // Clear the counts
    counts.iter_mut().for_each(|x| *x = 0);

    let awidth = width + 2;
    for (i, j) in (0..height).zip(0..width) {
        let val = board[i * width + j];
        counts[(i + 1 + 1) * awidth + j + 1] += val;
        counts[(i - 1 + 1) * awidth + j + 1] += val;
        counts[(i + 1) * awidth + j + 1 + 1] += val;
        counts[(i + 1) * awidth + j - 1 + 1] += val;
        counts[(i + 1 + 1) * awidth + (j + 1) + 1] += val;
        counts[(i - 1 + 1) * awidth + (j - 1) + 1] += val;
        counts[(i - 1 + 1) * awidth + j + 1 + 1] += val;
        counts[(i + 1 + 1) * awidth + (j - 1) + 1] += val;
    }
}

pub fn run(
    iterations: usize,
    board: &mut [i32],
    width: usize,
    height: usize,
    _: &[i32],
    _: usize,
    _: usize,
) {
    let counts_size = (width + 2) * (height + 2);
    let mut counts = vec![0; counts_size];

    for _ in 0..iterations {
        compute_counts(&mut counts, board, width, height);

        for (i, j) in (0..height).zip(0..width) {
            let coord = i * width + j;
            let currentvalue = board[coord];
            let neighbors = counts[(i + 1) * (width + 2) + j + 1];
            board[coord] = match (currentvalue, neighbors) {
                (1, 2) | (1, 3) | (0, 3) => 1,
                _ => 0,
            };
        }
    }
}
