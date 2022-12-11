struct Rectangle {
    length: u32,
    width: u32,
    height: u32,
}

impl Rectangle {
    fn get_paper_dimensions(&self) -> u32 {
        (2 * self.length * self.width)
            + (2 * self.width * self.height)
            + (2 * self.height * self.length)
            + self.get_smallest_area()
    }

    fn get_smallest_area(&self) -> u32 {
        [
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ]
        .into_iter()
        .min()
        .unwrap()
    }

    fn get_ribbon_length(&self) -> u32 {
        let mut smallest = [self.length, self.width, self.height];
        smallest.sort();

        let first = smallest.get(0).unwrap();
        let second = smallest.get(1).unwrap();

        let mut initial = first + first + second + second;

        initial += self.length * self.width * self.height;

        initial
    }

    fn build(input: &str) -> Self {
        let mut split = input.trim().split('x');
        let length = split.next().unwrap().parse::<u32>().unwrap();
        let width = split.next().unwrap().parse::<u32>().unwrap();
        let height = split.next().unwrap().parse::<u32>().unwrap();

        Self {
            length,
            width,
            height,
        }
    }
}

fn solution_1(input: &str) -> u32 {
    let rectangles = input.trim().lines().map(|line| Rectangle::build(line));

    rectangles.fold(0, |curr, accum| curr + accum.get_paper_dimensions())
}

fn solution_2(input: &str) -> u32 {
    let rectangles = input.trim().lines().map(|line| Rectangle::build(line));

    rectangles.fold(0, |curr, accum| curr + accum.get_ribbon_length())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        assert_eq!(58, solution_1("2x3x4"));
        assert_eq!(43, solution_1("1x1x10"));
    }

    #[test]
    fn solution_1_input() {
        let input = include_str!("input/day2.txt");
        assert_eq!(1588178, solution_1(input));
    }

    #[test]
    fn solution_2_example() {
        assert_eq!(34, solution_2("2x3x4"));
        assert_eq!(14, solution_2("1x1x10"));
    }

    #[test]
    fn solution_2_input() {
        let input = include_str!("input/day2.txt");
        assert_eq!(3783758, solution_2(input));
    }
}
