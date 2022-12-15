use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{complete::line_ending, streaming::char},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

fn parse_grid(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        nom::character::complete::i32,
        char(','),
        nom::character::complete::i32,
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<(Command, ((i32, i32), (i32, i32)))>> {
    let (_, result) = separated_list1(
        line_ending,
        separated_pair(
            alt((
                map(tag("toggle"), |_| Command::Toggle),
                map(tag("turn off"), |_| Command::TurnOff),
                map(tag("turn on"), |_| Command::TurnOn),
            )),
            tag(" "),
            separated_pair(parse_grid, tag(" through "), parse_grid),
        ),
    )(input)?;

    Ok((input, result))
}

fn solution_1(input: &str) -> u32 {
    let (_, contents) = parse(input).unwrap();

    let mut lights_map = HashMap::new();

    for (command, (from, to)) in contents {
        let min_x = from.0.min(to.0);
        let max_x = from.0.max(to.0);

        let min_y = from.1.min(to.1);
        let max_y = from.1.max(to.1);

        for (x, y) in (min_x..=max_x).cartesian_product(min_y..=max_y) {
            let entry = lights_map.entry((x, y)).or_insert(false);

            match command {
                Command::Toggle => {
                    *entry = !*entry;
                }
                Command::TurnOn => {
                    *entry = true;
                }
                Command::TurnOff => {
                    *entry = false;
                }
            }
        }
    }

    (lights_map.values().filter(|&x| *x == true).count()) as u32
}

fn solution_2(input: &str) -> u32 {
    let (_, contents) = parse(input).unwrap();

    let mut lights_map = HashMap::new();

    for (command, (from, to)) in contents {
        let min_x = from.0.min(to.0);
        let max_x = from.0.max(to.0);

        let min_y = from.1.min(to.1);
        let max_y = from.1.max(to.1);

        for (x, y) in (min_x..=max_x).cartesian_product(min_y..=max_y) {
            let entry = lights_map.entry((x, y)).or_insert(0u32);

            match command {
                Command::Toggle => {
                    *entry += 2;
                }
                Command::TurnOn => {
                    *entry += 1;
                }
                Command::TurnOff => {
                    *entry = (*entry as i32 - 1).max(0) as u32;
                }
            }
        }
    }

    lights_map.values().sum::<u32>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        assert_eq!(1_000_000, solution_1("turn on 0,0 through 999,999"));
        assert_eq!(1000, solution_1("toggle 0,0 through 999,0"));
    }

    #[test]
    fn solution_1_input() {
        let input = include_str!("input/day6.txt");
        assert_eq!(999_999, solution_1(input));
    }

    #[test]
    fn solution_2_example() {
        assert_eq!(2_000_000, solution_2("toggle 0,0 through 999,999"));
        assert_eq!(1, solution_2("turn on 0,0 through 0,0"));
    }

    #[test]
    fn solution_2_input() {
        let input = include_str!("input/day6.txt");
        assert_eq!(14_687_245, solution_2(input));
    }
}
