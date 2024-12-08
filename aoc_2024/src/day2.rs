fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .collect()
        })
        .collect()
}

fn solution_1(input: &str) -> usize {
    let input = parse_input(input);

    input
        .into_iter()
        .filter(|row| {
            let is_increasing = row[1] > row[0];
            for window in row.windows(2) {
                if window[1] > window[0] && !is_increasing {
                    return false;
                }

                if window[1] < window[0] && is_increasing {
                    return false;
                }

                if window[0] == window[1] {
                    return false;
                }

                let diff = window[0].abs_diff(window[1]);

                if diff < 1 || diff > 3 {
                    return false;
                }
            }

            true
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(2, solution_1(input));
    }

    #[test]
    fn solution_1_real() {
        let input = include_str!("input_files/day2.txt");

        assert_eq!(356, solution_1(input));
    }
}
