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

    input.into_iter().filter(|row| is_safe(row)).count()
}

fn solution_2(input: &str) -> usize {
    let rows = parse_input(input).into_iter().collect::<Vec<Vec<u32>>>();

    let safe_rows = rows
        .iter()
        .filter(|&row| is_safe(row))
        .collect::<Vec<&Vec<u32>>>();

    let unsafe_rows = rows
        .iter()
        .filter(|&row| safe_rows.iter().all(|&safe_row| row != safe_row))
        .filter_map(|row| sanitize_row(row))
        .collect::<Vec<Vec<u32>>>();

    unsafe_rows.len() + safe_rows.len()
}

fn is_safe(row: &[u32]) -> bool {
    let is_increasing = row[1] > row[0];
    row.windows(2).all(|row| {
        ((is_increasing && row[1] > row[0]) || (!is_increasing && row[1] < row[0]))
            && row[1].abs_diff(row[0]) <= 3
    })
}

fn sanitize_row(row: &[u32]) -> Option<Vec<u32>> {
    for index in 0..row.len() {
        let mut vec = Vec::from(row);

        vec.remove(index);

        if is_safe(&vec) {
            return Some(vec);
        }
    }

    None
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

    #[test]
    fn solution_2_example() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(4, solution_2(input));
    }

    #[test]
    fn solution_2_real() {
        let input = include_str!("input_files/day2.txt");

        assert_eq!(413, solution_2(input));
    }
}
