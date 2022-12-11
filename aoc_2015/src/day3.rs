use std::{collections::HashSet, ops::Add};

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Position(i32, i32);

impl From<char> for Direction {
    fn from(input: char) -> Self {
        match input {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Invalid character specified"),
        }
    }
}

impl From<&Direction> for Position {
    fn from(direction: &Direction) -> Self {
        match direction {
            Direction::Up => Position(0, 1),
            Direction::Down => Position(0, -1),
            Direction::Left => Position(-1, 0),
            Direction::Right => Position(1, 0),
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn visit(instructions: &Vec<Direction>) -> Vec<Position> {
    let mut visited = vec![Position(0, 0)];

    for instruction in instructions {
        let instruction = Position::from(instruction);
        let last = visited.last().unwrap().clone();

        visited.push(last + instruction);
    }

    visited
}

fn visit_duo(instructions: &Vec<Direction>) -> Vec<Position> {
    let mut visited = vec![Position(0, 0)];

    for direction in instructions {
        let visited_length = visited.len();
        let reference_pos = if visited_length < 3 {
            Position(0, 0)
        } else {
            visited[visited_length - 2].clone()
        };
        
        visited.push(reference_pos + Position::from(direction));
    }

    visited
}

fn solution_1(input: &str) -> usize {
    let instructions = input
        .trim()
        .lines()
        .flat_map(|line| line.chars().map(|char| Direction::from(char)))
        .collect::<Vec<_>>();

    let visited = visit(&instructions);

    visited.into_iter().collect::<HashSet<Position>>().len()
}

fn solution_2(input: &str) -> usize {
    let instructions = input
        .trim()
        .lines()
        .flat_map(|line| line.chars().map(|char| Direction::from(char)))
        .collect::<Vec<_>>();

    let visited = visit_duo(&instructions);

    visited.into_iter().collect::<HashSet<Position>>().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        assert_eq!(2, solution_1(">"));
        assert_eq!(4, solution_1("^>v<"));
        assert_eq!(2, solution_1("^v^v^v^v^v"));
    }

    #[test]
    fn solution_1_input() {
        let input = include_str!("input/day3.txt");
        assert_eq!(2565, solution_1(input));
    }

    #[test]
    fn solution_2_example() {
        assert_eq!(3, solution_2("^v"));
        assert_eq!(3, solution_2("^>v<"));
        assert_eq!(11, solution_2("^v^v^v^v^v"));
    }

    #[test]
    fn solution_2_input() {
        let input = include_str!("input/day3.txt");
        assert_eq!(2639, solution_2(input));
    }
}
