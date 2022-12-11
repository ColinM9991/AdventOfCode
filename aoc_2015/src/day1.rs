struct Floors<'a> {
    instructions: std::slice::Iter<'a, u8>,
    floor: i32,
}

impl<'a> Iterator for Floors<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.instructions.next().map(|floor| {
            match floor {
                b'(' => self.floor += 1,
                b')' => self.floor -= 1,
                _ => unreachable!(),
            }

            self.floor
        })
    }
}

impl<'a> From<&'a str> for Floors<'a> {
    fn from(input: &'a str) -> Self {
        Self {
            instructions: input.trim().as_bytes().iter(),
            floor: 0,
        }
    }
}

fn solution_1(input: &str) -> i32 {
    let floors = Floors::from(input);

    let result = floors.collect::<Vec<_>>();

    result.into_iter().last().unwrap()
}

fn solution_2(input: &str) -> Option<usize> {
    let floors = Floors::from(input);

    let result = floors.collect::<Vec<_>>();

    result.into_iter().position(|x| x < 0).map(|x| x + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        assert_eq!(0, solution_1("(())"));
        assert_eq!(0, solution_1("()()"));
        assert_eq!(3, solution_1("((("));
        assert_eq!(3, solution_1("(()(()("));
        assert_eq!(3, solution_1("))((((("));
        assert_eq!(-1, solution_1("())"));
        assert_eq!(-1, solution_1("))("));
        assert_eq!(-3, solution_1(")))"));
        assert_eq!(-3, solution_1(")())())"));
    }

    #[test]
    fn solution_1_input() {
        let input = include_str!("input/day1.txt");

        assert_eq!(74, solution_1(input));
    }

    #[test]
    fn solution_2_example() {
        assert_eq!(None, solution_2("(())"));
        assert_eq!(None, solution_2("()()"));
        assert_eq!(None, solution_2("((("));
        assert_eq!(None, solution_2("(()(()("));
        assert_eq!(Some(1), solution_2("))((((("));
        assert_eq!(Some(3), solution_2("())"));
        assert_eq!(Some(1), solution_2("))("));
        assert_eq!(Some(1), solution_2(")))"));
        assert_eq!(Some(1), solution_2(")())())"));
    }

    #[test]
    fn solution_2_input() {
        let input = include_str!("input/day1.txt");

        assert_eq!(Some(1795), solution_2(input));
    }
}
