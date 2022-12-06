fn input_to_bytes(input: &str) -> impl Iterator<Item = (u8, u8)> + '_ {
    input.trim().lines().filter_map(|line| {
        let mut split = line.split_whitespace();

        Some((
            split.next()?.as_bytes().first()? - (b'A' - 1),
            split.next()?.as_bytes().first()? - (b'X' - 1),
        ))
    })
}

fn solution_1(input: &str) -> u32 {
    input_to_bytes(input)
        .map(|(left, right)| get_score(left, right))
        .sum::<u32>()
}

fn solution_2(input: &str) -> u32 {
    input_to_bytes(input)
        .map(|(left, right)| get_score(left, 1 + (right + left) % 3))
        .sum::<u32>()
}

fn get_score(left: u8, right: u8) -> u32 {
    (((4 + right - left) % 3 * 3) + right) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        let input = "A Y
        B X
        C Z";

        assert_eq!(15, solution_1(input));
    }

    #[test]
    fn solution_1_input() {
        let input = include_str!("input/day2.txt");

        assert_eq!(15523, solution_1(input));
    }

    #[test]
    fn solution_2_example() {
        let input = "A Y
        B X
        C Z";

        assert_eq!(12, solution_2(input));
    }

    #[test]
    fn solution_2_input() {
        let input = include_str!("input/day2.txt");

        assert_eq!(15702, solution_2(input));
    }
}
