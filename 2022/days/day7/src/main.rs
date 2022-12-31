use std::{cell::RefCell, rc::Rc};

use framework::*;

const TOTAL_DISK_SPACE: u64 = 70_000_000;
const MIN_NEEDED_DISK_SPACE: u64 = 30_000_000;

type DirStack = Vec<Rc<RefCell<Directory>>>;

struct Solution;

impl SolutionProvider for Solution {
    type Output = u64;

    fn part1(input: &String) -> Result<u64> {
        let root = FileNode::parse(&input);

        let mut sizes = Vec::new();
        root.calculate_rm_sizes(&mut sizes, Some(100_000));

        let result = sizes.iter().sum();

        Ok(result)
    }
    fn part2(input: &String) -> Result<u64> {
        let root = FileNode::parse(&input);

        let mut sizes = Vec::new();
        root.calculate_rm_sizes(&mut sizes, None);

        let remaining_space = TOTAL_DISK_SPACE - sizes.last().unwrap();
        let min_size_to_delete = MIN_NEEDED_DISK_SPACE - remaining_space;

        let result = sizes
            .iter()
            .filter(|&&size| size >= min_size_to_delete)
            .min()
            .unwrap();

        Ok(*result)
    }
}

#[derive(Debug, Clone)]
enum FileNode {
    Directory(Rc<RefCell<Directory>>),
    File(File),
}

impl FileNode {
    pub fn parse(input: &String) -> Self {
        let root_dir = Rc::new(RefCell::new(Directory {
            name: "/".to_owned(),
            nodes: Vec::new(),
        }));

        let mut dir_stack: DirStack = vec![root_dir.clone()];

        for line in input.lines().skip(1) {
            match line {
                s if s.starts_with("$ cd") => FileNode::handle_cd(line, &mut dir_stack),
                s if s.starts_with("$ ls") => (),
                _ => FileNode::handle_new_node(line, &mut dir_stack),
            }
        }

        FileNode::Directory(root_dir)
    }

    pub fn calculate_rm_sizes(&self, sizes: &mut Vec<u64>, max_dir_size: Option<u64>) -> u64 {
        let mut total_size = 0;

        match self {
            FileNode::Directory(dir) => {
                for node in &dir.borrow().nodes {
                    total_size += node.calculate_rm_sizes(sizes, max_dir_size);
                }

                if max_dir_size.is_none() || total_size <= max_dir_size.unwrap() {
                    sizes.push(total_size);
                }

                return total_size;
            }
            FileNode::File(file) => {
                return file.size;
            }
        }
    }

    fn handle_cd(line: &str, dir_stack: &mut DirStack) {
        let new_dir = line.split_whitespace().nth(2).unwrap();

        match new_dir {
            ".." => {
                dir_stack.pop();
            }
            dir => {
                let dir = Rc::new(RefCell::new(Directory {
                    name: dir.into(),
                    nodes: Vec::new(),
                }));

                let node = FileNode::Directory(dir.clone());
                dir_stack.last().unwrap().borrow_mut().nodes.push(node);
                dir_stack.push(dir);
            }
        };
    }

    fn handle_new_node(line: &str, dir_stack: &mut DirStack) {
        let mut split = line.split_whitespace();
        let first = split.next().unwrap();
        let last = split.next().unwrap();

        match first {
            "dir" => (),
            size => {
                let node = FileNode::File(File {
                    name: last.into(),
                    size: size.parse::<u64>().unwrap(),
                });
                dir_stack.last().unwrap().borrow_mut().nodes.push(node);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Directory {
    #[allow(dead_code)]
    name: String,
    nodes: Vec<FileNode>,
}
#[derive(Debug, Clone)]
struct File {
    #[allow(dead_code)]
    name: String,
    size: u64,
}

aoc_main!(Solution; input);
aoc_test!(imp, Solution, input, 1306611, 13210366);
