pub fn solution_1(input: &str) -> u32 {
    let solution: u32 = input
        .trim()
        .lines()
        .map(|line| line.trim()) // Trim the input lines to remove whitespace
        .map(|line| line.split_at(line.len() / 2)) // Split the input in half
        .flat_map(|(left, right)| left.chars().find(|&y| right.contains(y))) // Find chars that intersect both left and right
        .filter_map(|input| match input.is_uppercase() {
            true => Some(input as u32 - 65 + 27), // If it's uppercase, get the byte representation + 27 (a-zA-Z, where A is 26, + 1 for the offset)
            false => Some(input as u32 - 97 + 1) // If it's lowercase, get the byte representation + 1 for the offset
        })
        .sum(); 
    
    solution
}

pub fn solution_2(input: &str) -> u32 {
    let solution = input
        .trim()
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .flat_map(|groups| groups[0].chars().find(|&x| groups[1].contains(x) && groups[2].contains(x)))
        .filter_map(|input| match input.is_uppercase() {
            true => Some(input as u32 - 65 + 27),
            false => Some(input as u32 - 97 + 1)
        })
        .sum(); 

    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(157, solution_1(input))
    }

    #[test]
    fn solution_1_input() {
        let input = include_str!("input/day3.txt");

        assert_eq!(7701, solution_1(input));
    }

    #[test]
    fn solution_2_example() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg";

        assert_eq!(18, solution_2(input));
    }

    #[test]
    fn solution_2_input() {
        let input = include_str!("input/day3.txt");

        assert_eq!(2644, solution_2(input));
    }
}
