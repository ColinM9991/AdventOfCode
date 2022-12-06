fn get_tree_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect::<Vec<Vec<u8>>>()
}

fn solution_1(input: &str) -> u32 {
    let mut outside_visible = 0;
    let mut inside_visible = 0;

    let tree_grid = get_tree_grid(input);

    for (y, _) in tree_grid.iter().enumerate() {
        for (x, &current_tree) in tree_grid[y].iter().enumerate() {
            if y == 0 || x == 0 || y == &tree_grid.len() - 1 || x == tree_grid[0].len() - 1 {
                outside_visible += 1;

                continue;
            }

            let above = tree_grid[..y]
                .iter()
                .map(|grid| &grid[x])
                .rev()
                .collect::<Vec<_>>();

            let left = tree_grid[y][..x].iter().rev().collect::<Vec<_>>();

            let below = tree_grid[y + 1..]
                .iter()
                .map(|grid| &grid[x])
                .collect::<Vec<_>>();

            let right = tree_grid[y][x + 1..].iter().collect::<Vec<_>>();

            if is_visible(current_tree, &above)
                || is_visible(current_tree, &left)
                || is_visible(current_tree, &below)
                || is_visible(current_tree, &right)
            {
                inside_visible += 1;
            }
        }
    }

    outside_visible + inside_visible
}

fn solution_2(input: &str) -> usize {
    let mut weight = 0;

    let tree_grid = get_tree_grid(input);

    for (y, _) in tree_grid.iter().enumerate() {
        for (x, &current_tree) in tree_grid[y].iter().enumerate() {
            if y == 0 || x == 0 || y == &tree_grid.len() - 1 || x == tree_grid[0].len() - 1 {
                continue;
            }

            let above = tree_grid[..y]
                .iter()
                .map(|grid| &grid[x])
                .rev()
                .collect::<Vec<_>>();

            let left = tree_grid[y][..x].iter().rev().collect::<Vec<_>>();

            let below = tree_grid[y + 1..]
                .iter()
                .map(|grid| &grid[x])
                .collect::<Vec<_>>();

            let right = tree_grid[y][x + 1..].iter().collect::<Vec<_>>();

            let above = get_visited(current_tree, &above);
            let left = get_visited(current_tree, &left);
            let below = get_visited(current_tree, &below);
            let right = get_visited(current_tree, &right);

            weight = weight.max(above * left * below * right);
        }
    }

    weight
}

fn is_visible(tree: u8, line_of_sight: &Vec<&u8>) -> bool {
    line_of_sight
        .iter()
        .all(|&neighboring_tree| neighboring_tree < &tree)
}

fn get_visited(tree: u8, line_of_sight: &Vec<&u8>) -> usize {
    let mut visited = 0;

    for &neighboring_tree in line_of_sight {
        visited += 1;
        if neighboring_tree >= &tree {
            break;
        }
    }

    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        let input = "30373
        25512
        65332
        33549
        35390";

        assert_eq!(21, solution_1(input));
    }

    #[test]
    fn solution_1_input() {
        let input = include_str!("input/day8.txt");

        assert_eq!(1676, solution_1(input));
    }

    #[test]
    fn solution_2_example() {
        let input = "30373
        25512
        65332
        33549
        35390";

        assert_eq!(8, solution_2(input));
    }

    #[test]
    fn solution_2_input() {
        let input = include_str!("input/day8.txt");

        assert_eq!(313200, solution_2(input));
    }
}
