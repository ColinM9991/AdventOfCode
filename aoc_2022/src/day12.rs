use pathfinding::prelude::astar;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Hash, Eq, PartialEq, Clone, Copy, PartialOrd, Ord, Debug)]
struct Position(i32, i32);

const LOWEST_ELEVATION: char = 'a';
const HIGHEST_ELEVATION: char = 'z';

impl Position {
    fn get_distance(&self, other: &Position) -> u32 {
        (self.0.abs_diff(other.0)) + (self.1.abs_diff(other.1)) as u32
    }

    fn neighbours(&self, elevations: &BTreeMap<Position, char>) -> Vec<(Position, u32)> {
        let current_elevation = elevations.get(self).unwrap().clone();
        let current_elevation = current_elevation as u8 + 1;

        let neighbors = vec![
            Position(self.0, self.1 - 1),
            Position(self.0, self.1 + 1),
            Position(self.0 - 1, self.1),
            Position(self.0 + 1, self.1),
        ];

        let neighbors = neighbors
            .into_iter()
            .filter(|neighbor| elevations.contains_key(neighbor))
            .collect::<Vec<_>>();

        let neighbors = neighbors
            .into_iter()
            .filter_map(|neighbor| {
                let neighbor_elevation = elevations.get(&neighbor).unwrap().clone();
                let neighbor_elevation = neighbor_elevation as u8;

                if neighbor_elevation > current_elevation {
                    None
                } else {
                    Some((neighbor, 1))
                }
            })
            .collect::<Vec<(Position, u32)>>();

        neighbors
    }
}

struct Grid {
    positions: Vec<BTreeSet<Position>>,
    elevations: BTreeMap<Position, char>,

    start_pos: Option<Position>,
    end_pos: Option<Position>,
}

impl Grid {
    fn new() -> Self {
        Self {
            positions: vec![],
            elevations: BTreeMap::new(),
            start_pos: None,
            end_pos: None,
        }
    }
}

fn parse_input(input: &str) -> Grid {
    let mut grid = Grid::new();

    for (row, line) in input.trim().lines().enumerate() {
        let chars = line.trim().chars();

        let mut set = BTreeSet::new();

        for (col, char) in chars.enumerate() {
            let position = Position(col.try_into().unwrap(), row.try_into().unwrap());
            grid.elevations.insert(
                position,
                match char {
                    'S' => 'a',
                    'E' => 'z',
                    char => char,
                },
            );

            match char {
                'S' => grid.start_pos = Some(position),
                'E' => grid.end_pos = Some(position),
                _ => {}
            }

            set.insert(position);
        }

        grid.positions.push(set);
    }
    grid
}

fn solution_1(input: &str) -> u32 {
    let grid = parse_input(input);

    let result = astar(
        &grid.start_pos.unwrap(),
        |p| p.neighbours(&grid.elevations),
        |p| p.get_distance(&grid.end_pos.unwrap()),
        |p| *p == grid.end_pos.unwrap(),
    );

    match result {
        Some(val) => val.1,
        None => 0,
    }
}

fn solution_2(input: &str) -> u32 {
    let grid = parse_input(input);
    
    let result = astar(
        &grid.start_pos.unwrap(),
        |p| p.neighbours(&grid.elevations),
        |p| p.get_distance(&grid.end_pos.unwrap()),
        |p| *p == grid.end_pos.unwrap(),
    );

    match result {
        Some(val) => val.1,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        let input = "Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi";

        assert_eq!(31, solution_1(input));
    }

    #[test]
    fn solution_1_input() {
        let input = include_str!("input/day12.txt");

        assert_eq!(339, solution_1(input));
    }
}
