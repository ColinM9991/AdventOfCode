use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq)]
enum FileType {
    File,
    Directory,
}

struct File<'a> {
    name: &'a str,
    size: Option<usize>,
    file_type: FileType,

    children: Option<Vec<Rc<RefCell<File<'a>>>>>,
    parent: Option<Rc<RefCell<File<'a>>>>,
}

impl<'a> File<'a> {
    fn add_child_item(&mut self, item: Rc<RefCell<File<'a>>>) {
        self.children.as_mut().unwrap().push(Rc::clone(&item));
    }

    fn all_children(&self) -> Option<&Vec<Rc<RefCell<File<'a>>>>> {
        self.children.as_ref()
    }

    fn get_child_item(&self, name: &str) -> Option<Rc<RefCell<File<'a>>>> {
        self.all_children()?
            .iter()
            .find(|&child| child.borrow().name == name)
            .map(|res| Rc::clone(res))
    }

    fn get_directories(&self) -> Option<Vec<Rc<RefCell<Self>>>> {
        Some(
            self.all_children()?
                .iter()
                .filter(|&child| child.borrow().file_type == FileType::Directory)
                .map(|res| Rc::clone(&res))
                .collect(),
        )
    }

    fn get_parent(&self) -> Option<Rc<RefCell<Self>>> {
        match &self.parent {
            Some(parent) => Some(Rc::clone(&parent)),
            None => unreachable!(),
        }
    }

    fn size(&self) -> usize {
        match self.file_type {
            FileType::File => self.size.unwrap(),
            FileType::Directory => self
                .all_children()
                .unwrap()
                .iter()
                .fold(0, |curr, acc| curr + acc.borrow().size()),
        }
    }

    fn total_size(&self) -> Vec<usize> {
        let mut sizes = vec![self.size()];

        match self.get_directories() {
            Some(dirs) => {
                let mut dir_sizes = dirs
                    .iter()
                    .flat_map(|x| x.borrow().total_size())
                    .collect::<Vec<_>>();
                sizes.append(&mut dir_sizes);
            }
            None => {}
        }

        sizes
    }

    fn parse(input: &str) -> Rc<RefCell<File>> {
        let parsed_commands = ParserResult::parse_input(input);
        let root = Rc::new(RefCell::new(File::default()));
        let mut cwd = Rc::clone(&root);

        for instruction in parsed_commands {
            match instruction {
                ParserResult::ChangeDirUp => {
                    let target_dir = cwd.borrow().get_parent().unwrap();

                    cwd = target_dir;
                }
                ParserResult::ChangeDir(dir) => {
                    let target_dir = cwd.borrow().get_child_item(dir).unwrap();

                    cwd = target_dir;
                }
                ParserResult::Directory(dir) => {
                    let directory = File {
                        name: dir,
                        file_type: FileType::Directory,
                        size: None,
                        parent: Some(Rc::clone(&cwd)),
                        children: Some(vec![]),
                    };

                    cwd.borrow_mut()
                        .add_child_item(Rc::new(RefCell::new(directory)));
                }
                ParserResult::File(file, size) => {
                    let file = File {
                        name: file,
                        file_type: FileType::File,
                        size: Some(size),
                        parent: Some(Rc::clone(&cwd)),
                        children: None,
                    };

                    cwd.borrow_mut().add_child_item(Rc::new(RefCell::new(file)));
                }
            }
        }

        root
    }
}

impl<'a> Default for File<'a> {
    fn default() -> Self {
        Self {
            name: "/",
            size: None,
            file_type: FileType::Directory,
            children: Some(vec![]),
            parent: None,
        }
    }
}

#[derive(Debug)]
enum ParserResult<'a> {
    ChangeDirUp,
    ChangeDir(&'a str),
    Directory(&'a str),
    File(&'a str, usize),
}

impl ParserResult<'static> {
    fn parse_input(input: &str) -> Vec<ParserResult> {
        input
            .trim()
            .lines()
            .skip(2)
            .filter_map(|line| {
                let split = line.trim().split_whitespace().collect::<Vec<_>>();

                match split[..] {
                    ["$", "cd", arg] => match arg {
                        "/" => None,
                        ".." => Some(ParserResult::ChangeDirUp),
                        dir => Some(ParserResult::ChangeDir(dir)),
                    },
                    ["$", "ls"] => None,
                    ["dir", second] => Some(ParserResult::Directory(second)),
                    [size, name] => Some(ParserResult::File(name, size.parse::<usize>().unwrap())),
                    _ => None,
                }
            })
            .collect::<Vec<_>>()
    }
}

fn solution_1(input: &str) -> usize {
    let file_system = File::parse(input);
    let file_system = file_system.borrow();

    let total_sizes = file_system.total_size();

    total_sizes.into_iter().filter(|&size| size <= 100000).sum()
}

fn solution_2(input: &str) -> Option<usize> {
    const MAX_SPACE: usize = 70_000_000;
    const REQUIRED_SPACE: usize = 30_000_000;

    let file_system = File::parse(input);
    let file_system = file_system.borrow();
    let total_size = file_system.size();

    let remaining_space = MAX_SPACE - total_size;
    let space_to_clear = REQUIRED_SPACE - remaining_space;

    file_system
        .total_size()
        .into_iter()
        .filter(|&size| size >= space_to_clear)
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        let input = "$ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k";

        assert_eq!(95437, solution_1(input));
    }

    #[test]
    fn solution_1_input() {
        let input = include_str!("input/day7.txt");

        assert_eq!(1543140, solution_1(input));
    }

    #[test]
    fn solution_2_example() {
        let input = "$ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k";

        assert_eq!(24933642, solution_2(input).unwrap());
    }

    #[test]
    fn solution_2_input() {
        let input = include_str!("input/day7.txt");

        assert_eq!(1117448, solution_2(input).unwrap());
    }
}
