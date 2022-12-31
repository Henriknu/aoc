use framework::*;

struct Solution;

impl SolutionProvider for Solution {
    type Output = u64;

    fn part1(input: &String) -> Result<u64> {
        let root = FileNode::parse(&input);

        dbg!(&root);

        let result = root.calculate_rm_size(100_000);

        Ok(result)
    }
    fn part2(input: &String) -> Result<u64> {
        todo!()
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
                _ => (),
            }
        }

        FileNode::Directory(root_dir)
    }

    pub fn calculate_rm_size(&self, max_dir_size: u64) -> u64 {
        let mut total_size = 0;

        match self {
            FileNode::Directory(dir) => {
                for node in &dir.nodes {
                    total_size += node.calculate_rm_size(max_dir_size);
                }
                if total_size > max_dir_size {
                    return 0;
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

        dbg!(&dir_stack);

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

aoc_main!(Solution; test);
aoc_test!(imp, Solution, input, 13052, 13693);
