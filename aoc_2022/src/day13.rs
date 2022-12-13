use std::iter::once;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    *,
};

#[derive(Debug, Eq, Clone)]
enum Packet {
    Nested(Vec<Packet>),
    Single(u32),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nested(left), Self::Nested(right)) => left == right,
            (Self::Single(left), Self::Single(right)) => left == right,
            (Self::Nested(left), Self::Single(right)) => *left == vec![Packet::Single(*right)],
            (Self::Single(left), Self::Nested(right)) => vec![Packet::Single(*left)] == *right,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Nested(left), Packet::Nested(right)) => left.cmp(&right),
            (Packet::Nested(left), Packet::Single(right)) => {
                left.cmp(&vec![Packet::Single(*right)])
            }
            (Packet::Single(left), Packet::Nested(right)) => {
                vec![Packet::Single(*left)].cmp(&right)
            }
            (Packet::Single(left), Packet::Single(right)) => left.cmp(&right),
        }
    }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    let nested_parser = delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]"))
        .map(|vec| Packet::Nested(vec));

    let primitive_parser = nom::character::complete::u32.map(|num| Packet::Single(num));

    let mut parser = alt((nested_parser, primitive_parser));

    parser(input)
}

fn parse(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    let newline_tag = tag("\n\n");

    let pairs_parser = separated_pair(parse_packet, newline, parse_packet).map(|(l, r)| (l, r));

    let mut parser = separated_list1(newline_tag, pairs_parser);

    parser(input)
}

fn solution_1(input: &str) -> usize {
    let (_, pairs) = parse(input).unwrap();

    pairs
        .into_iter()
        .enumerate()
        .filter(|(_, (l, r))| l.cmp(&r) == std::cmp::Ordering::Less)
        .map(|(index, _)| index + 1)
        .sum()
}

fn solution_2(input: &str) -> usize {
    let (_, pairs) = parse(input).unwrap();

    let mut flat_mapped = pairs
        .into_iter()
        .flat_map(|(left, right)| once(left).chain(once(right)))
        .collect::<Vec<_>>();

    let two = Packet::Nested(vec![Packet::Nested(vec![Packet::Single(2)])]);
    let six = Packet::Nested(vec![Packet::Nested(vec![Packet::Single(6)])]);

    flat_mapped.push(two.clone());
    flat_mapped.push(six.clone());

    flat_mapped.sort();

    let two_index = flat_mapped
        .iter()
        .enumerate()
        .find(|(_, item)| *item == &two)
        .map(|(index, _)| index + 1)
        .unwrap();

    let six_index = flat_mapped
        .iter()
        .enumerate()
        .find(|(_, item)| *item == &six)
        .map(|(index, _)| index + 1)
        .unwrap();

    two_index * six_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        let input = include_str!("input/example/day13.txt");

        assert_eq!(13, solution_1(input));
    }

    #[test]
    fn solution_1_input() {
        let input = include_str!("input/day13.txt");

        assert_eq!(5825, solution_1(input));
    }

    #[test]
    fn solution_2_example() {
        let input = include_str!("input/example/day13.txt");

        assert_eq!(140, solution_2(input));
    }

    #[test]
    fn solution_2_input() {
        let input = include_str!("input/day13.txt");

        assert_eq!(24477, solution_2(input));
    }
}
