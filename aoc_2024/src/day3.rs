use regex::Regex;

fn solution_1(input: &str) -> u32 {
    let re = Regex::new("mul\\((\\d+),(\\d+)\\)").unwrap();

    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [first, second])| {
            (
                first.parse::<u32>().unwrap(),
                second.parse::<u32>().unwrap(),
            )
        })
        .fold(0, |acc, (first, second)| acc + (first * second))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(161, solution_1(input));
    }

    #[test]
    fn solution_1_real() {
        let input = include_str!("input_files/day3.txt");

        assert_eq!(0, solution_1(input));
    }
}
