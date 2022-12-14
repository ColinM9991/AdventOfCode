use nom::{
    character::complete::{line_ending, newline},
    combinator::map,
    multi::separated_list1,
    sequence::pair,
    IResult,
};

fn parse_group(input: &str) -> IResult<&str, u32> {
    map(
        separated_list1(newline, nom::character::complete::u32),
        |val| val.into_iter().sum(),
    )(input)
}

fn parse_whole(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(pair(line_ending, line_ending), parse_group)(input)
}

fn solution_1(input: &str) -> Option<u32> {
    get_calories(input).max()
}

fn solution_2(input: &str) -> u32 {
    let mut calories = get_calories(input).collect::<Vec<u32>>();

    calories.sort();
    calories.reverse();
    calories.into_iter().take(3).sum::<u32>()
}

fn get_calories(input: &str) -> impl Iterator<Item = u32> + '_ {
    let (_, result) = parse_whole(input).unwrap();

    result.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(24000, solution_1(input).unwrap());
    }

    #[test]
    fn solution_1_input() {
        let input = include_str!("input/day1.txt");

        assert_eq!(67633, solution_1(input).unwrap());
    }

    #[test]
    fn solution_2_example() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(45000, solution_2(input));
    }

    #[test]
    fn solution_2_input() {
        let input = include_str!("input/day1.txt");

        assert_eq!(199628, solution_2(input));
    }
}
