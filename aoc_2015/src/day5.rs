use itertools::Itertools;

const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];
const DISALLOWED_CHARS: &[&str] = &["ab", "cd", "pq", "xy"];

fn old_matcher(input: &str) -> bool {
    let mut has_doubles = false;
    let mut has_invalid = false;

    for (first, second) in input.chars().tuple_windows() {
        let string = format!("{}{}", first, second);
        if DISALLOWED_CHARS.contains(&string.as_str()) {
            has_invalid = true;
            break;
        }

        if first == second {
            has_doubles = true;
        }
    }

    !has_invalid
        && input
            .to_lowercase()
            .chars()
            .filter(|char| VOWELS.contains(&char))
            .count()
            >= 3
        && has_doubles
}

fn new_matcher(input: &str) -> bool {
    let mut has_matching_separator = false;
    let mut has_matching_pair = false;

    for (first, second, third) in input.chars().tuple_windows() {
        if first == third {
            has_matching_separator = true;
        }

        let pair = format!("{}{}", first, second);
        let matches = input.matches(&pair);

        if matches.count() >= 2 && second != third {
            has_matching_pair = true;
        }
    }

    has_matching_pair && has_matching_separator
}

fn solution_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| old_matcher(line))
        .count()
}

fn solution_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| new_matcher(line))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        assert_eq!(1, solution_1("ugknbfddgicrmopn"));
        assert_eq!(1, solution_1("aaa"));
        assert_eq!(0, solution_1("jchzalrnumimnmhp"));
        assert_eq!(0, solution_1("haegwjzuvuyypxyu"));
        assert_eq!(0, solution_1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn solution_1_input() {
        let input = include_str!("input/day5.txt");
        assert_eq!(238, solution_1(input));
    }

    #[test]
    fn solution_2_example() {
        assert_eq!(1, solution_2("qjhvhtzxzqqjkmpb"));
        assert_eq!(1, solution_2("xxyxx"));
        assert_eq!(0, solution_2("uurcxstgmygtbstg"));
        assert_eq!(0, solution_2("ieodomkazucvgmuy"));
    }

    #[test]
    fn solution_2_input() {
        let input = include_str!("input/day5.txt");
        assert_eq!(1, solution_2(input));
    }
}
