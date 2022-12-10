use std::collections::HashSet;

struct Direction<'a>(&'a str, i32);

#[derive(Clone, Copy, PartialEq, PartialOrd, Hash, Eq)]
struct Vector(i32, i32);

fn get_positions(input: &str, followers: u8) -> Vec<Vector> {
    let moves: Vec<Direction> = input
        .trim()
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let dir = split.next().unwrap();
            let moves = split.next().unwrap().parse::<i32>().unwrap();

            Direction(dir, moves)
        })
        .collect();

    let mut snake = vec![Vector(0, 0); followers.into()];

    let mut visited_positions = vec![];
    visited_positions.push(Vector(0, 0));

    for _move in moves {
        for _ in 1..=_move.1 {
            move_snake(&mut snake, _move.0);

            visited_positions.push(snake.last().unwrap().clone());
        }
    }

    visited_positions
}

fn move_snake(snake: &mut Vec<Vector>, dir: &str) {
    let turn_pos = match dir.to_uppercase().as_str() {
        "U" => Vector(0, 1),
        "D" => Vector(0, -1),
        "L" => Vector(-1, 0),
        "R" => Vector(1, 0),
        _ => unreachable!(),
    };

    let head = snake.get_mut(0).unwrap();

    head.0 += turn_pos.0;
    head.1 += turn_pos.1;

    for index in 1..snake.iter().len() {
        let x = snake[index - 1].0 - snake[index].0;
        let y = snake[index - 1].1 - snake[index].1;

        if x.abs() > 1 || y.abs() > 1 {
            let tail = &mut snake[index];

            tail.0 += x.signum();
            tail.1 += y.signum();
        }
    }
}

fn solution_1(input: &str) -> i32 {
    get_positions(input, 2)
        .into_iter()
        .collect::<HashSet<Vector>>()
        .iter()
        .count()
        .try_into()
        .unwrap()
}

fn solution_2(input: &str) -> i32 {
    get_positions(input, 10)
        .into_iter()
        .collect::<HashSet<Vector>>()
        .iter()
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        let input = "R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2";

        assert_eq!(13, solution_1(input));
    }

    #[test]
    fn solution_1_input() {
        let input = include_str!("input/day9.txt");

        assert_eq!(5878, solution_1(input));
    }

    #[test]
    fn solution_2_example() {
        let input = "R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2";

        assert_eq!(1, solution_2(input));
    }

    #[test]
    fn solution_2_input() {
        let input = include_str!("input/day9.txt");

        assert_eq!(2405, solution_2(input));
    }
}
