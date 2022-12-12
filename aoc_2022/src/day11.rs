use std::collections::VecDeque;

use itertools::Itertools;

#[derive(Debug)]
enum OperationValue {
    By(u64),
    ByOriginal,
}

#[derive(Debug)]
enum Operation {
    Add(OperationValue),
    Multiply(OperationValue),
}

#[derive(Debug)]
struct Test {
    value: u64,
    true_target: usize,
    false_target: usize,
}

#[derive(Debug)]
struct Monkey {
    id: u32,
    gifts: VecDeque<u64>,
    inspections: u64,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn new(id: u32, gifts: Vec<u64>, operation: Operation, test: Test) -> Self {
        Self {
            id,
            gifts: VecDeque::from_iter(gifts.into_iter()),
            inspections: 0,
            operation,
            test,
        }
    }

    fn inspect_item<F>(&mut self, f: &F) -> u64
    where
        F: Fn(u64) -> u64,
    {
        self.inspections += 1;
        let gift = self.gifts.pop_front().unwrap();

        let mut gift = match &self.operation {
            Operation::Add(val) => match val {
                OperationValue::By(by) => gift + *by,
                OperationValue::ByOriginal => gift + gift,
            },
            Operation::Multiply(val) => match val {
                OperationValue::By(by) => gift * *by,
                OperationValue::ByOriginal => gift * gift,
            },
        };

        gift = f(gift);

        gift
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let monkey_groups = input.trim().split("\n\n");

    let mut monkeys = vec![];

    for group_item in monkey_groups {
        let mut group_lines = group_item.trim().lines().map(|line| line.trim());

        let monkey_id = group_lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .take(1)
            .next()
            .unwrap()
            .replace(":", "")
            .parse::<u32>()
            .unwrap();

        let starting_items = group_lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(2)
            .map(|item| item.replace(",", "").parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let mut operation = group_lines.next().unwrap().split_whitespace().skip(4);

        let operation_sign = operation.next().unwrap();
        let value = operation.next().unwrap();

        let value = match value {
            "old" => OperationValue::ByOriginal,
            val => OperationValue::By(val.parse::<u64>().unwrap()),
        };

        let operation = match operation_sign {
            "+" => Operation::Add(value),
            "*" => Operation::Multiply(value),
            _ => panic!(),
        };

        let test = group_lines.collect::<Vec<_>>();
        let mut test = test.iter();
        let condition = test
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let true_expression = test
            .next()
            .unwrap()
            .trim()
            .chars()
            .last()
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap();

        let false_expression = test
            .next()
            .unwrap()
            .trim()
            .chars()
            .last()
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap();

        let monkey = Monkey::new(
            monkey_id,
            starting_items,
            operation,
            Test {
                value: condition,
                true_target: true_expression,
                false_target: false_expression,
            },
        );

        monkeys.push(monkey);
    }

    monkeys
}

fn play_game<F>(monkeys: &mut Vec<Monkey>, rounds: u32, f: F) -> u64
where
    F: Fn(u64) -> u64,
{
    for _ in 1..=rounds {
        for index in 0..monkeys.len() {
            for _ in 0..monkeys[index].gifts.len() {
                let monkey = monkeys.get_mut(index).unwrap();
                let item = monkey.inspect_item(&f);

                let recipient_monkey_id = if item % (monkey.test.value as u64) == 0 {
                    monkey.test.true_target
                } else {
                    monkey.test.false_target
                };

                let recipient = monkeys.get_mut(recipient_monkey_id).unwrap();
                recipient.gifts.push_back(item);
            }
        }
    }

    monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn solution_1(input: &str) -> u64 {
    let mut monkeys = parse_input(input);

    let result = play_game(&mut monkeys, 20, |gift| gift / 3);

    result
}

fn solution_2(input: &str) -> u64 {
    let mut monkeys = parse_input(input);

    let lcm = monkeys
        .iter()
        .map(|monkey| monkey.test.value as u64)
        .product::<u64>();

    let result = play_game(&mut monkeys, 10000, |gift| gift % lcm);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        let example_input = include_str!("input/example/day11.txt");
        assert_eq!(10605, solution_1(example_input));
    }

    #[test]
    fn solution_1_input() {
        let example_input = include_str!("input/day11.txt");
        assert_eq!(121450, solution_1(example_input));
    }

    #[test]
    fn solution_2_example() {
        let example_input = include_str!("input/example/day11.txt");
        assert_eq!(2713310158, solution_2(example_input));
    }

    #[test]
    fn solution_2_input() {
        let example_input = include_str!("input/day11.txt");
        assert_eq!(28244037010, solution_2(example_input));
    }
}
