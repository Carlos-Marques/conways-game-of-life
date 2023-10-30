use std::ops::{BitAnd, BitOr, Not};
use std::simd::SimdPartialEq;
use std::simd::{i32x4, SimdInt};

fn compute_counts(counts: &mut [i32], board: &[i32], width: usize, height: usize) {
    // Clear the counts
    counts.iter_mut().for_each(|x| *x = 0);

    let awidth = width + 2;
    for (i, j) in (0..height).zip((0..(width / 4)).map(|x| x * 4)) {
        let mut c = i32x4::splat(0);

        c += i32x4::from_slice(&board[((i + 1 + 1) * awidth + j + 1)..]);
        c += i32x4::from_slice(&board[((i - 1 + 1) * awidth + j + 1)..]);
        c += i32x4::from_slice(&board[((i + 1) * awidth + j + 1 + 1)..]);
        c += i32x4::from_slice(&board[((i + 1) * awidth + j - 1 + 1)..]);
        c += i32x4::from_slice(&board[((i + 1 + 1) * awidth + (j + 1) + 1)..]);
        c += i32x4::from_slice(&board[((i - 1 + 1) * awidth + (j - 1) + 1)..]);
        c += i32x4::from_slice(&board[((i - 1 + 1) * awidth + j + 1 + 1)..]);
        c += i32x4::from_slice(&board[((i + 1 + 1) * awidth + (j - 1) + 1)..]);

        c.copy_to_slice(&mut counts[((i + 1) * (width + 2) + j + 1)..]);
    }
}

fn update_cells(neighbours: &[i32], board: &mut [i32], width: usize, height: usize) {
    let awidth = width + 2;
    for (i, j) in (0..height).zip((0..(width / 4)).map(|x| x * 4)) {
        let neighbour_count = i32x4::from_slice(&neighbours[((i + 1) * awidth + j + 1)..]);
        let old_state = i32x4::from_slice(&board[((i + 1) * awidth + j + 1)..]);

        let is_alive = old_state.simd_eq(i32x4::splat(1));
        let two_neighbours = neighbour_count.simd_eq(i32x4::splat(2));
        let three_neighbours = neighbour_count.simd_eq(i32x4::splat(3));

        let remains_alive = is_alive.bitand(two_neighbours.bitor(three_neighbours));
        let becomes_alive = is_alive.not().bitand(three_neighbours);
        let new_state_mask = remains_alive.bitor(becomes_alive);

        // to_int returns 0 or -1;
        let new_state = new_state_mask.to_int().abs();

        new_state.copy_to_slice(&mut board[((i + 1) * awidth + j + 1)..]);
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
    debug_assert_eq!(width % 4, 0);

    let counts_size = (width + 2) * (height + 2);
    let mut counts = vec![0; counts_size];

    for _ in 0..iterations {
        compute_counts(&mut counts, board, width, height);
        update_cells(&counts[..], board, width, height);
    }
}
