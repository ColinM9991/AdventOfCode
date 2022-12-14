use std::collections::HashSet;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Debug, Clone, Copy, Hash, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

fn parse_line(input: &str) -> IResult<&str, impl Iterator<Item = Position>> {
    let input = input.trim();
    let (input, pairs) = separated_list1(
        tag(" -> "),
        separated_pair(complete::i32, tag(","), complete::i32).map(|(x, y)| (x, y)),
    )(input)?;

    let pairs = pairs.into_iter().tuple_windows().flat_map(|(left, right)| {
        let (lx, ly) = (left.0, left.1);
        let (rx, ry) = (right.0, right.1);

        let (min_x, max_x) = (lx.min(rx), lx.max(rx));
        let (min_y, max_y) = (ly.min(ry), ly.max(ry));

        let x_range = min_x..=max_x;
        let y_range = min_y..=max_y;

        x_range
            .cartesian_product(y_range)
            .map(|(l, r)| Position::new(l, r))
    });

    Ok((input, pairs))
}

fn parse(input: &str) -> IResult<&str, HashSet<Position>> {
    let (input, res) = separated_list1(line_ending, parse_line)(input)?;

    let res = res.into_iter().flatten().collect();

    Ok((input, res))
}

fn drop_sand(rocks: &HashSet<Position>, max_depth: i32, sand_pos: Position) -> Position {
    if sand_pos.y == max_depth + 1 {
        return sand_pos;
    }

    let down = sand_pos.down();
    let left = sand_pos.left();
    let right = sand_pos.right();

    match (
        rocks.contains(&down),
        rocks.contains(&left),
        rocks.contains(&right),
    ) {
        (false, _, _) => drop_sand(rocks, max_depth, down),
        (_, false, _) => drop_sand(rocks, max_depth, left),
        (_, _, false) => drop_sand(rocks, max_depth, right),
        _ => sand_pos,
    }
}

fn solution_1(input: &str) -> u32 {
    let (_, mut rocks) = parse(input).unwrap();
    let rocks_length = rocks.len();

    let Position { y: max_y, .. } = rocks
        .iter()
        .max_by(|left, right| left.y.cmp(&right.y))
        .unwrap()
        .clone();

    let starting_point = Position::new(500, 0);

    loop {
        let sand = drop_sand(&rocks, max_y, starting_point);
        if sand.y == max_y + 1 {
            break;
        }

        rocks.insert(sand);
    }

    (rocks.len() - rocks_length).try_into().unwrap()
}

fn solution_2(input: &str) -> u32 {
    let (_, mut rocks) = parse(input).unwrap();
    let rocks_length = rocks.len();

    let Position { y: max_y, .. } = rocks
        .iter()
        .max_by(|left, right| left.y.cmp(&right.y))
        .unwrap()
        .clone();

    let starting_point = Position::new(500, 0);

    while !rocks.contains(&starting_point) {
        let sand = drop_sand(&rocks, max_y, starting_point);

        rocks.insert(sand);
    }

    (rocks.len() - rocks_length).try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        let input = "498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9";

        assert_eq!(24, solution_1(input));
    }

    #[test]
    fn solution_1_input() {
        let input = include_str!("input/day14.txt");

        assert_eq!(1072, solution_1(input));
    }

    #[test]
    fn solution_2_example() {
        let input = "498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9";

        assert_eq!(93, solution_2(input));
    }

    #[test]
    fn solution_2_input() {
        let input = include_str!("input/day14.txt");

        assert_eq!(24659, solution_2(input));
    }
}
