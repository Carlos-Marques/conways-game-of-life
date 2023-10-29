use clap::Parser;
use itertools::Itertools;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Parser)]
struct Opts {
    #[clap(name = "input")]
    input: PathBuf,

    #[clap(long = "no_relief")]
    no_relief: bool,

    #[clap(short, long, default_value = "20")]
    number_of_rounds: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    let input_path = opts.input;
    let mut file = File::open(&input_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let monkeys = parse_monkeys(contents.as_str());

    let result = calculate_monkey_business(opts.number_of_rounds, opts.no_relief, monkeys);

    println!("{}", result);
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Test {
    divisible_by: usize,
    true_monkey_index: usize,
    false_monkey_index: usize,
}

struct Monkey {
    items: Vec<usize>,
    operation: Arc<dyn Fn(usize) -> usize + Send + Sync>,
    test: Test,
    number_of_inspections: usize,
}

impl Clone for Monkey {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
            operation: Arc::clone(&self.operation),
            test: self.test.clone(),
            number_of_inspections: self.number_of_inspections.clone(),
        }
    }
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("operation", &format!("function pointer"))
            .field("test", &self.test)
            .finish()
    }
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items && self.test == other.test
    }
}

fn parse_test(test_lines: &[&str]) -> Test {
    let divisible_by: usize = test_lines[0]
        .trim()
        .strip_prefix("Test: divisible by ")
        .unwrap()
        .parse()
        .unwrap();
    let true_monkey_index: usize = test_lines[1]
        .trim()
        .strip_prefix("If true: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();
    let false_monkey_index: usize = test_lines[2]
        .trim()
        .strip_prefix("If false: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();
    Test {
        divisible_by,
        true_monkey_index,
        false_monkey_index,
    }
}

fn parse_operations(operation_line: &str) -> Arc<dyn Fn(usize) -> usize + Send + Sync> {
    let operation_line = operation_line
        .trim()
        .strip_prefix("Operation: new = ")
        .unwrap();

    match operation_line {
        op if op.contains("* old") => Arc::new(move |old| old * old),
        op if op.contains("*") => {
            let factor: usize = op.split(" * ").last().unwrap().parse().unwrap();
            Arc::new(move |old| old * factor)
        }
        op => {
            let addend: usize = op.split(" + ").last().unwrap().parse().unwrap();
            Arc::new(move |old| old + addend)
        }
    }
}

fn parse_starting_items(starting_items_line: &str) -> Vec<usize> {
    let items_line = starting_items_line
        .trim()
        .strip_prefix("Starting items: ")
        .unwrap();
    items_line.split(", ").map(|s| s.parse().unwrap()).collect()
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let monkey_descriptions = input.split("Monkey").filter(|s| !s.trim().is_empty());

    let monkeys = monkey_descriptions
        .map(|monkey_description| {
            let lines: Vec<&str> = monkey_description.lines().collect();

            let items = parse_starting_items(lines[1]);
            let operation = parse_operations(lines[2]);
            let test = parse_test(&lines[3..=5]);

            Monkey {
                items,
                operation,
                test,
                number_of_inspections: 0,
            }
        })
        .collect();

    monkeys
}
fn play_round(
    mut monkeys: Vec<Monkey>,
    no_relief: bool,
    divisible_by_product: usize,
) -> Vec<Monkey> {
    for index in 0..monkeys.len() {
        let monkey = monkeys[index].clone();

        for mut item in monkey.items {
            item %= divisible_by_product;

            let new_worry = if no_relief {
                (monkey.operation)(item)
            } else {
                (monkey.operation)(item) / 3
            };

            match new_worry % monkey.test.divisible_by == 0 {
                true => monkeys[monkey.test.true_monkey_index].items.push(new_worry),
                false => monkeys[monkey.test.false_monkey_index]
                    .items
                    .push(new_worry),
            }

            monkeys[index].number_of_inspections += 1;
        }

        monkeys[index].items = Vec::new();
    }

    monkeys
}

fn calculate_monkey_business(n_rounds: usize, no_relief: bool, mut monkeys: Vec<Monkey>) -> usize {
    let divisible_by_product = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product::<usize>();

    for _ in 0..n_rounds {
        monkeys = play_round(monkeys, no_relief, divisible_by_product);
    }

    let inspections: Vec<usize> = monkeys
        .into_iter()
        .map(|monkey| monkey.number_of_inspections)
        .sorted()
        .collect();

    let len = inspections.len();
    inspections[len - 1] * inspections[len - 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monkey_business() {
        let input = r#"
        Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

        Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

        Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
        "#;

        let monkeys = parse_monkeys(input);

        assert_eq!(
            vec![
                Monkey {
                    items: vec![79, 98],
                    operation: Arc::new(|old| old * 19),
                    test: Test {
                        divisible_by: 23,
                        true_monkey_index: 2,
                        false_monkey_index: 3,
                    },
                    number_of_inspections: 0
                },
                Monkey {
                    items: vec![54, 65, 75, 74],
                    operation: Arc::new(|old| old + 6),
                    test: Test {
                        divisible_by: 19,
                        true_monkey_index: 2,
                        false_monkey_index: 0,
                    },
                    number_of_inspections: 0
                },
                Monkey {
                    items: vec![79, 60, 97],
                    operation: Arc::new(|old| old * old),
                    test: Test {
                        divisible_by: 13,
                        true_monkey_index: 1,
                        false_monkey_index: 3,
                    },
                    number_of_inspections: 0
                },
                Monkey {
                    items: vec![74],
                    operation: Arc::new(|old| old + 3),
                    test: Test {
                        divisible_by: 17,
                        true_monkey_index: 0,
                        false_monkey_index: 1,
                    },
                    number_of_inspections: 0
                },
            ],
            monkeys
        );

        let monkey_business = calculate_monkey_business(20, false, monkeys);

        assert_eq!(monkey_business, 10605);
    }

    #[test]
    fn test_monkey_business_no_relief() {
        let input = r#"
        Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

        Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

        Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
        "#;

        let monkeys = parse_monkeys(input);

        assert_eq!(
            vec![
                Monkey {
                    items: vec![79, 98],
                    operation: Arc::new(|old| old * 19),
                    test: Test {
                        divisible_by: 23,
                        true_monkey_index: 2,
                        false_monkey_index: 3,
                    },
                    number_of_inspections: 0
                },
                Monkey {
                    items: vec![54, 65, 75, 74],
                    operation: Arc::new(|old| old + 6),
                    test: Test {
                        divisible_by: 19,
                        true_monkey_index: 2,
                        false_monkey_index: 0,
                    },
                    number_of_inspections: 0
                },
                Monkey {
                    items: vec![79, 60, 97],
                    operation: Arc::new(|old| old * old),
                    test: Test {
                        divisible_by: 13,
                        true_monkey_index: 1,
                        false_monkey_index: 3,
                    },
                    number_of_inspections: 0
                },
                Monkey {
                    items: vec![74],
                    operation: Arc::new(|old| old + 3),
                    test: Test {
                        divisible_by: 17,
                        true_monkey_index: 0,
                        false_monkey_index: 1,
                    },
                    number_of_inspections: 0
                },
            ],
            monkeys
        );

        let monkey_business = calculate_monkey_business(10000, true, monkeys);

        assert_eq!(monkey_business, 2713310158);
    }
}
