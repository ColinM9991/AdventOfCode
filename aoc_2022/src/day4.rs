fn solution_1(input: &str) -> u32 {
    let pairs = input
        .trim()
        .lines()
        .map(|line| line.trim().split(',').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut ans: u32 = 0;
    for pair in pairs {
        let (first, second) = pair[0].split_once('-').unwrap();
        let (third, fourth) = pair[1].split_once('-').unwrap();

        let (s1, e1, s2, e2) = (
            first.parse::<u32>().unwrap(),
            second.parse::<u32>().unwrap(),
            third.parse::<u32>().unwrap(),
            fourth.parse::<u32>().unwrap(),
        );

        if s1 <= s2 && e2 <= e1 || s2 <= s1 && e1 <= e2 {
            ans += 1;
        }
    }

    ans
}

fn solution_2(input: &str) -> u32 {
    let pairs = input
        .trim()
        .lines()
        .map(|line| line.trim().split(',').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut ans: u32 = 0;
    for pair in pairs {
        let (first, second) = pair[0].split_once('-').unwrap();
        let (third, fourth) = pair[1].split_once('-').unwrap();

        let (s1, e1, s2, e2) = (
            first.parse::<u32>().unwrap(),
            second.parse::<u32>().unwrap(),
            third.parse::<u32>().unwrap(),
            fourth.parse::<u32>().unwrap(),
        );

        if e1 >= s2 && e2 >= s1 {
            ans += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        let input = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";

        assert_eq!(2, solution_1(input));
    }

    #[test]
    fn solution_1_input() {
        let input = include_str!("input/day4.txt");

        assert_eq!(651, solution_1(input));
    }

    #[test]
    fn solution_2_input() {
        let input = include_str!("input/day4.txt");

        assert_eq!(956, solution_2(input));
    }
}
