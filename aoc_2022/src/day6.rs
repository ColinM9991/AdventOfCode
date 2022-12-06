use std::collections::BTreeSet;

fn get_marker_index(input: &str, window_size: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .find(|(_, item)| {
            let seq = item.iter().collect::<BTreeSet<_>>();

            seq.len() == window_size
        })
        .map(|(index, _)| index + window_size)
}

fn solution_1(input: &str) -> Option<usize> {
    get_marker_index(input, 4)
}

fn solution_2(input: &str) -> Option<usize> {
    get_marker_index(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        assert_eq!(solution_1("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 5);
        assert_eq!(solution_1("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 6);
        assert_eq!(solution_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 10);
        assert_eq!(solution_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 11);
    }

    #[test]
    fn solution_1_input() {
        assert_eq!(solution_1(include_str!("input/day6.txt")).unwrap(), 1109);
    }

    #[test]
    fn solution_2_example() {
        assert_eq!(solution_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(), 19);
        assert_eq!(solution_2("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 23);
        assert_eq!(solution_2("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 23);
        assert_eq!(solution_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 29);
        assert_eq!(solution_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 26);
    }

    #[test]
    fn solution_2_input() {
        assert_eq!(solution_2(include_str!("input/day6.txt")).unwrap(), 3965);
    }
}
