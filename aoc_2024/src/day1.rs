use std::collections::HashMap;

pub fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|e| {
            let mut split = e.split_whitespace();
            (
                split.next().map(|x| x.parse::<u32>().unwrap()).unwrap(),
                split.next().map(|x| x.parse::<u32>().unwrap()).unwrap(),
            )
        })
        .unzip()
}

pub fn sort_input(mut input: (Vec<u32>, Vec<u32>)) -> (Vec<u32>, Vec<u32>) {
    input.0.sort();
    input.1.sort();

    (input.0, input.1)
}

fn solution_1(input: &str) -> u32 {
    let split_input = sort_input(parse_input(input));
    let left = split_input.0;
    let right = split_input.1;

    left.into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

fn solution_2(input: &str) -> u32 {
    let parsed_input = parse_input(input);
    let left = parsed_input.0;
    let right = parsed_input.1;

    let mut hashmap = HashMap::new();

    left.into_iter().fold(0, |val, item| {
        let value = hashmap
            .entry(item)
            .or_insert(right.iter().filter(|&&y| item == y).count() as u32);

        val + (item * *value)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn solution_1_example() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(11, solution_1(input));
    }

    #[test]
    pub fn solution_1_real() {
        let input = include_str!("input_files/day1.txt");

        assert_eq!(1320851, solution_1(input));
    }

    #[test]
    pub fn solution_2_example() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(31, solution_2(input));
    }

    #[test]
    pub fn solution_2_real() {
        let input = include_str!("input_files/day1.txt");

        assert_eq!(26859182, solution_2(input));
    }
}
