use md5;

fn compute_input(input: &str, zeroes: u8) -> u32 {
    let padded_zeroes = str::repeat("0", zeroes.into());
    
    let mut index = 0;
    loop {
        let digest = md5::compute(format!("{input}{index}"));
        let digest = format!("{:x}", digest);
        if digest.starts_with(&padded_zeroes) {
            return index;
        }

        index += 1;
    }
}

fn solution_1(input: &str) -> u32 {
    compute_input(input, 5)
}

fn solution_2(input: &str) -> u32 {
    compute_input(input, 6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        assert_eq!(609043, solution_1("abcdef"));
        assert_eq!(1048970, solution_1("pqrstuv"));
    }

    #[test]
    fn solution_1_input() {
        assert_eq!(282749, solution_1("yzbqklnj"));
    }

    #[test]
    fn solution_2_example() {
        assert_eq!(6742839, solution_2("abcdef"));
        assert_eq!(5714438, solution_2("pqrstuv"));
    }

    #[test]
    fn solution_2_input() {
        assert_eq!(9962624, solution_2("yzbqklnj"));
    }
}