use regex::Regex;

// To be refactored

fn solution_1(input: &Vec<Vec<&str>>, instructions: &str) -> String {
    let mut input = input.to_vec();

    let instructions = instructions
        .trim()
        .lines()
        .map(|instruction| instruction.trim())
        .collect::<Vec<&str>>();

    let re = Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();

    for instruction in instructions {
        let matches = re.captures(instruction.trim()).unwrap();
        let (quantity, source, destination) = (
            matches.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            matches.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            matches.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        );

        let source = input.get_mut(source - 1).unwrap();
        let mut removed_items = source.drain(..quantity).collect::<Vec<&str>>();

        let destination = input.get_mut(destination - 1).unwrap();

        destination.reverse();
        destination.append(&mut removed_items);
        destination.reverse();
    }

    let res = input
        .into_iter()
        .map(|value| value.iter().next().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("");

    res
}

fn solution_2(input: &Vec<Vec<&str>>, instructions: &str) -> String {
    let mut input = input.to_vec();

    let instructions = instructions
        .trim()
        .lines()
        .map(|instruction| instruction.trim())
        .collect::<Vec<&str>>();

    let re = Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();

    let index = 0;
    for instruction in instructions {
        let matches = re.captures(instruction.trim()).unwrap();
        let (quantity, source, destination) = (
            matches.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            matches.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            matches.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        );

        let source = input.get_mut(source - 1).unwrap();
        let mut removed_items = source.drain(..quantity).collect::<Vec<&str>>();

        if index % 1 == 0 {
            removed_items.reverse();
        }

        let destination = input.get_mut(destination - 1).unwrap();

        destination.reverse();
        destination.append(&mut removed_items);
        destination.reverse();
    }

    let res = input
        .into_iter()
        .map(|value| value.iter().next().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("");

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        let mut input = vec![vec!["N", "Z"], vec!["D", "C", "M"], vec!["P"]];

        let instructions = "move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2";

        assert_eq!("CMZ", solution_1(&mut input, instructions));
    }

    #[test]
    fn solution_1_input() {
        let mut input = vec![
            vec!["P", "Z", "M", "T", "R", "C", "N"],
            vec!["Z", "B", "S", "T", "N", "D"],
            vec!["G", "T", "C", "F", "R", "Q", "H", "M"],
            vec!["Z", "R", "G"],
            vec!["H", "R", "N", "Z"],
            vec!["D", "L", "Z", "P", "W", "S", "H", "F"],
            vec!["M", "G", "C", "R", "Z", "D", "W"],
            vec!["Q", "Z", "W", "H", "L", "F", "J", "S"],
            vec!["N", "W", "P", "Q", "S"]
        ];

        let instructions = include_str!("input/day5.txt");

        assert_eq!("RTGWZTHLD", solution_1(&mut input, instructions));
    }

    #[test]
    fn solution_2_example() {
        let mut input = vec![vec!["N", "Z"], vec!["D", "C", "M"], vec!["P"]];

        let instructions = "move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2";

        assert_eq!("MCD", solution_2(&mut input, instructions));
    }

    #[test]
    fn solution_2_input() {
        let mut input = vec![
            vec!["P", "Z", "M", "T", "R", "C", "N"],
            vec!["Z", "B", "S", "T", "N", "D"],
            vec!["G", "T", "C", "F", "R", "Q", "H", "M"],
            vec!["Z", "R", "G"],
            vec!["H", "R", "N", "Z"],
            vec!["D", "L", "Z", "P", "W", "S", "H", "F"],
            vec!["M", "G", "C", "R", "Z", "D", "W"],
            vec!["Q", "Z", "W", "H", "L", "F", "J", "S"],
            vec!["N", "W", "P", "Q", "S"]
        ];

        let instructions = include_str!("input/day5.txt");

        assert_eq!("STHGRZZFR", solution_2(&mut input, instructions));
    }
}
