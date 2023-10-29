use clap::Parser;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Parser)]
struct Opts {
    #[clap(name = "input")]
    input: PathBuf,

    #[clap(long = "delete", short = 'd')]
    delete: bool,
}

#[derive(PartialEq, Eq, Debug)]
struct FileSize {
    name: String,
    size: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    let input_path = opts.input;
    let file = File::open(&input_path).unwrap();
    let reader = BufReader::new(file);

    let directories = get_directories(reader);

    let directory_sizes = get_directory_sizes(directories);

    let result = match opts.delete {
        false => sum_directory_sizes(directory_sizes, 100000),
        true => find_min_directory_size(directory_sizes, 70000000, 30000000),
    };

    println!("{}", result);
}

fn sum_directory_sizes(directory_sizes: HashMap<PathBuf, usize>, filter: usize) -> usize {
    directory_sizes
        .into_iter()
        .map(|(_, size)| size)
        .filter(|&size| size < filter)
        .sum()
}

fn find_min_directory_size(
    directory_sizes: HashMap<PathBuf, usize>,
    total_space: usize,
    needed_space: usize,
) -> usize {
    let free_space = total_space - directory_sizes.get(&PathBuf::from("/")).unwrap();

    directory_sizes
        .into_iter()
        .map(|(_, size)| size)
        .filter(|size| size + free_space > needed_space)
        .min()
        .unwrap()
}

fn get_directories<R: BufRead>(reader: R) -> HashMap<PathBuf, Vec<FileSize>> {
    let (_, directories) = reader.lines().map(|line| line.unwrap()).fold(
        (PathBuf::new(), HashMap::<PathBuf, Vec<FileSize>>::new()),
        |(mut current_dir, mut directories), line| {
            let command = line.split_whitespace().collect::<Vec<_>>();

            match command.get(0) {
                Some(&"$") => match command.get(1) {
                    Some(&"cd") => match command.get(2) {
                        Some(&"..") => {
                            current_dir.pop();
                        }
                        Some(&"/") => {
                            current_dir = PathBuf::from("/");
                        }
                        Some(&directory_name) => {
                            current_dir.push(directory_name);
                        }
                        _ => {
                            panic!("No argument on cd!");
                        }
                    },
                    _ => {}
                },
                Some(&arg) => {
                    let file = match arg {
                        "dir" => FileSize {
                            size: 0,
                            name: command[1].to_string(),
                        },
                        file_size => {
                            let file_name = command[1];
                            FileSize {
                                size: file_size.parse::<usize>().unwrap(),
                                name: file_name.to_string(),
                            }
                        }
                    };

                    directories
                        .entry(current_dir.clone())
                        .or_insert_with(Vec::new)
                        .push(file);
                }
                None => {}
            }

            (current_dir, directories)
        },
    );

    directories
}

fn get_directory_sizes(directories: HashMap<PathBuf, Vec<FileSize>>) -> HashMap<PathBuf, usize> {
    directories
        .keys()
        .sorted()
        .rev()
        .fold(HashMap::new(), |mut acc, path| {
            let directory_size: usize = directories
                .get(path)
                .unwrap()
                .into_iter()
                .map(|file| match file {
                    FileSize { name, size: 0 } => {
                        let mut dir_name = path.clone();
                        dir_name.push(name);

                        acc.get(&dir_name).unwrap()
                    }
                    FileSize { name: _, size } => size,
                })
                .sum();

            acc.insert(path.clone(), directory_size);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_directories() {
        let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";
        let reader = BufReader::new(input.as_bytes());

        let directories = get_directories(reader);
        let directory_sizes = get_directory_sizes(directories);

        let sum_directories: usize = sum_directory_sizes(directory_sizes, 100000);

        assert_eq!(sum_directories, 95437);
    }

    #[test]
    fn test_find_min_directory_size() {
        let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";
        let reader = BufReader::new(input.as_bytes());

        let directories = get_directories(reader);
        let directory_sizes = get_directory_sizes(directories);

        let min_directory_size: usize =
            find_min_directory_size(directory_sizes, 70000000, 30000000);

        assert_eq!(min_directory_size, 24933642);
    }
}
