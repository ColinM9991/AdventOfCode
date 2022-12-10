enum Instruction {
    AddX(i32),
    NoOp,
}

impl Instruction {
    fn get_cycle(&self) -> i32 {
        match &self {
            Instruction::NoOp => 1,
            Instruction::AddX(_) => 2,
        }
    }

    fn value(&self) -> i32 {
        match self {
            Instruction::NoOp => 0,
            Instruction::AddX(val) => *val,
        }
    }
}

struct Cycle {
    cycle: i32,
    value: i32,
}

impl Cycle {
    fn new(cycle: i32, value: i32) -> Self {
        Self { cycle, value }
    }
}

fn get_cycles(input: &str) -> Vec<Cycle> {
    let instructions = input
        .trim()
        .lines()
        .map(|line| {
            let mut split = line.trim().split_whitespace();

            match split.next().unwrap().to_lowercase().as_str() {
                "noop" => Instruction::NoOp,
                "addx" => Instruction::AddX(split.next().unwrap().parse::<i32>().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();

    let mut value = 1;
    let mut iterations = vec![];

    let mut cycle = 1;
    for (_, instruction) in instructions.iter().enumerate() {
        let instruction_cycle = instruction.get_cycle();

        for cycle_index in 1..=instruction_cycle {
            iterations.push(Cycle::new(cycle, value));

            if cycle_index == instruction_cycle {
                value += instruction.value();
            }

            cycle += 1;
        }
    }

    iterations
}

fn solution_1(input: &str) -> i32 {
    let cycles = get_cycles(input);

    let ranges = &[20, 60, 100, 140, 180, 220];
    let mut result = 0;
    for range in ranges {
        let cycle: &Cycle = &cycles[range - 1];

        let cycle_result = cycle.cycle * cycle.value;

        result += cycle_result;
    }

    result
}

fn solution_2(input: &str) -> String {
    let characters = [" ", "█"];
    let cycles = get_cycles(input);

    let mut screen = String::default();

    for cycle in cycles {
        let sprite = cycle.value;
        let column = (cycle.cycle - 1) % 40;

        let char = characters[((sprite - column).abs() < 2) as usize];
        screen.push_str(char);
    }

    let result = screen
        .chars()
        .collect::<Vec<_>>()
        .chunks(40)
        .map(|char| char.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{result}");

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        let input = "addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop";

        assert_eq!(13140, solution_1(input));
    }

    #[test]
    fn solution_1_input() {
        let input = include_str!("input/day10.txt");

        assert_eq!(13180, solution_1(input));
    }

    #[test]
    fn solution_2_example() {
        let input = "addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop";

        let result = "██  ██  ██  ██  ██  ██  ██  ██  ██  ██  \n███   ███   ███   ███   ███   ███   ███ \n████    ████    ████    ████    ████    \n█████     █████     █████     █████     \n██████      ██████      ██████      ████\n███████       ███████       ███████";

        assert_eq!(result, solution_2(input).trim());
    }

    #[test]
    fn solution_2_input() {
        let input = include_str!("input/day10.txt");

        let result = "████ ████ ████  ██  █  █   ██  ██  ███  
█       █ █    █  █ █  █    █ █  █ █  █ 
███    █  ███  █    ████    █ █  █ ███  
█     █   █    █    █  █    █ ████ █  █ 
█    █    █    █  █ █  █ █  █ █  █ █  █ 
████ ████ █     ██  █  █  ██  █  █ ███  ";

        assert_eq!(result.trim(), solution_2(input).trim());
    }
}
