use framework::*;

const TOTAL_DISK_SPACE: u64 = 70_000_000;
const MIN_NEEDED_DISK_SPACE: u64 = 30_000_000;

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
    Directory(Directory),
    File(File),
}

impl FileNode {
    pub fn parse(input: &String) -> Self {
        let mut root_dir = Directory {
            name: "/".to_owned(),
            nodes: Vec::new(),
        };

        let mut dir_stack = vec![root_dir.name.clone()];

        for line in input.lines().skip(1) {
            match line {
                s if s.starts_with("$ cd") => {
                    FileNode::handle_cd(line, &mut root_dir, &mut dir_stack)
                }
                s if s.starts_with("$ ls") => (),
                _ => FileNode::handle_new_node(line, &mut root_dir, &mut dir_stack),
            }
        }

        FileNode::Directory(root_dir)
    }

    pub fn calculate_rm_sizes(&self, sizes: &mut Vec<u64>, max_dir_size: Option<u64>) -> u64 {
        let mut total_size = 0;

        match self {
            FileNode::Directory(dir) => {
                for node in &dir.nodes {
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

    fn handle_cd(line: &str, root: &mut Directory, dir_stack: &mut Vec<String>) {
        let new_dir = line.split_whitespace().nth(2).unwrap();

        match new_dir {
            ".." => {
                dir_stack.pop();
            }
            dir => {
                let node = FileNode::Directory(Directory {
                    name: dir.into(),
                    nodes: Vec::new(),
                });

                let current_dir = root.traverse_to_current_dir(dir_stack);

                current_dir.nodes.push(node);
                dir_stack.push(dir.into());
            }
        };
    }

    fn handle_new_node(line: &str, root_dir: &mut Directory, dir_stack: &[String]) {
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

                let current_dir = root_dir.traverse_to_current_dir(dir_stack);
                current_dir.nodes.push(node);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    nodes: Vec<FileNode>,
}
impl Directory {
    fn traverse_to_current_dir(&mut self, dir_stack: &[String]) -> &mut Directory {
        let mut stack = dir_stack.iter().skip(1);
        let mut current_dir = self;
        while let Some(next_dir_name) = stack.next() {
            current_dir = current_dir
                .nodes
                .iter_mut()
                .filter_map(|node| match node {
                    FileNode::Directory(dir) => {
                        if dir.name == *next_dir_name {
                            Some(dir)
                        } else {
                            None
                        }
                    }
                    FileNode::File(_) => None,
                })
                .next()
                .unwrap();
        }

        current_dir
    }
}
#[derive(Debug, Clone)]
struct File {
    name: String,
    size: u64,
}

aoc_main!(Solution; input);
aoc_test!(imp, Solution, input, 1306611, 13210366);
