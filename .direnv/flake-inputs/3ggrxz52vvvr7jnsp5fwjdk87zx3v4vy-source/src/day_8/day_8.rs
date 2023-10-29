use clap::Parser;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Parser)]
struct Opts {
    #[clap(name = "input")]
    input: PathBuf,

    #[clap(long = "scenic", short = 's')]
    scenic: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    let input_path = opts.input;
    let file = File::open(&input_path).unwrap();
    let reader = BufReader::new(file);

    let trees = get_tree_heights(reader);

    let height = trees.len();
    let width = trees[0].len();

    let result = match opts.scenic {
        false => count_visible(&trees, width, height),
        true => get_scenic_score(&trees, width, height),
    };

    println!("{}", result)
}

fn get_scenic_score<'a>(trees: &'a Vec<Vec<Tree>>, width: usize, height: usize) -> usize {
    (1..height - 1)
        .flat_map(|row| {
            (1..width - 1).map(move |column| {
                let current_height = trees[row][column].height;
                let condition = move |tree: &Tree| tree.height < current_height;

                let (right_trees, left_trees, top_trees, bottom_trees) =
                    scan_directions(&trees, row, column, width, height, &condition, true);

                right_trees.count() * left_trees.count() * top_trees.count() * bottom_trees.count()
            })
        })
        .max()
        .unwrap()
}

fn count_visible<'a>(trees: &'a Vec<Vec<Tree>>, width: usize, height: usize) -> usize {
    (1..height - 1)
        .flat_map(|row| {
            (1..width - 1).map(move |column| {
                let current_height = trees[row][column].height;
                let condition = move |tree: &Tree| tree.height < current_height;

                let (right_trees, left_trees, top_trees, bottom_trees) =
                    scan_directions(&trees, row, column, width, height, &condition, false);

                right_trees
                    .last()
                    .map_or(false, |tree| tree.row == row && tree.column == width - 1)
                    || left_trees
                        .last()
                        .map_or(false, |tree| tree.row == row && tree.column == 0)
                    || top_trees
                        .last()
                        .map_or(false, |tree| tree.row == 0 && tree.column == column)
                    || bottom_trees.last().map_or(false, |tree| {
                        tree.row == height - 1 && tree.column == column
                    })
            })
        })
        .filter(|&visible| visible)
        .count()
        + width * 2
        + height * 2
        - 4
}

fn scan_directions<'a, F>(
    trees: &'a Vec<Vec<Tree>>,
    row: usize,
    column: usize,
    width: usize,
    height: usize,
    condition: &'a F,
    keep_last: bool,
) -> (
    impl Iterator<Item = &'a Tree>,
    impl Iterator<Item = &'a Tree>,
    impl Iterator<Item = &'a Tree>,
    impl Iterator<Item = &'a Tree>,
)
where
    F: Fn(&Tree) -> bool,
{
    let right_indexes = (column + 1..width).map(move |c| (row, c));
    let right_trees = scan_trees(&trees, right_indexes, condition, keep_last);

    let left_indexes = (0..column).rev().map(move |c| (row, c));
    let left_trees = scan_trees(&trees, left_indexes, condition, keep_last);

    let top_indexes = (0..row).rev().map(move |r| (r, column));
    let top_trees = scan_trees(&trees, top_indexes, condition, keep_last);

    let bottom_indexes = (row + 1..height).map(move |r| (r, column));
    let bottom_trees = scan_trees(&trees, bottom_indexes, condition, keep_last);

    (right_trees, left_trees, top_trees, bottom_trees)
}

#[derive(Debug)]
struct Tree {
    row: usize,
    column: usize,
    height: usize,
}

fn scan_trees<'a, F>(
    trees: &'a Vec<Vec<Tree>>,
    indexes: impl IntoIterator<Item = (usize, usize)>,
    condition: F,
    keep_last: bool,
) -> impl Iterator<Item = &'a Tree>
where
    F: Fn(&Tree) -> bool,
{
    indexes
        .into_iter()
        .map(move |(row, column)| &trees[row][column])
        .scan(true, move |state, tree| {
            if *state {
                if condition(tree) {
                    Some(Some(tree))
                } else {
                    *state = false;
                    if keep_last {
                        Some(Some(tree))
                    } else {
                        None
                    }
                }
            } else {
                None
            }
        })
        .take_while(|option| option.is_some())
        .filter_map(|option| option)
}

fn get_tree_heights<R: BufRead>(reader: R) -> Vec<Vec<Tree>> {
    reader
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.unwrap()
                .chars()
                .enumerate()
                .map(|(column, c)| Tree {
                    row,
                    column,
                    height: c.to_digit(10).unwrap() as usize,
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_visible_trees() {
        let input = "30373\n25512\n65332\n33549\n35390";
        let reader = BufReader::new(input.as_bytes());
        let trees = get_tree_heights(reader);

        let height = trees.len();
        let width = trees[0].len();

        let number_visible = count_visible(&trees, width, height);

        assert_eq!(number_visible, 21);
    }

    #[test]
    fn test_get_best_scenic_view() {
        let input = "30373\n25512\n65332\n33549\n35390";
        let reader = BufReader::new(input.as_bytes());
        let trees = get_tree_heights(reader);

        let height = trees.len();
        let width = trees[0].len();

        let best_scenic_score = get_scenic_score(&trees, width, height);

        assert_eq!(best_scenic_score, 8);
    }
}
