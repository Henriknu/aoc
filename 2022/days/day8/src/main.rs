use std::ops::Range;

use framework::*;

struct Solution;

impl SolutionProvider for Solution {
    type Output = u64;

    fn part1(input: &String) -> Result<u64> {
        let trees = parse_trees(&input);

        let count = count_trees(&trees);

        Ok(count)
    }
    fn part2(input: &String) -> Result<u64> {
        let trees = parse_trees(&input);

        let score = calculate_max_scenic_score(&trees);

        Ok(score)
    }
}

fn parse_trees(input: &str) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut inner = Vec::with_capacity(line.len());
        for char in line.chars() {
            inner.push(char.to_digit(10).unwrap());
        }
        result.push(inner);
    }
    result
}

// Part 1

fn count_trees(trees: &Vec<Vec<u32>>) -> u64 {
    let mut count = 0;

    // Add all trees on edge, as they are all visible

    let vertical_len = trees.len();
    let horizontal_len = trees[0].len();
    count += (vertical_len * 2 + 2 * horizontal_len - 4) as u64;

    // Add any interior tree which is visible from at least one side
    // Note: Ensure to skip trees already added from the edge

    let mut outer = trees.iter().enumerate().skip(1).peekable();

    while let Some((outer_index, tree_line)) = outer.next() {
        if outer.peek().is_none() {
            break;
        }

        let mut inner = tree_line.iter().enumerate().skip(1).peekable();

        while let Some((inner_index, tree)) = inner.next() {
            if inner.peek().is_none() {
                break;
            }

            if can_see(*tree, &trees, outer_index, inner_index) {
                count += 1
            }
        }
    }

    count
}

fn can_see(tree: u32, trees: &Vec<Vec<u32>>, outer_index: usize, inner_index: usize) -> bool {
    let up_line = 0..outer_index;
    let down_line = (outer_index + 1)..trees.len();
    let left_line = 0..inner_index;
    let right_line = (inner_index + 1)..trees[0].len();

    can_see_vertical(tree, &trees, up_line, inner_index)
        || can_see_vertical(tree, &trees, down_line, inner_index)
        || can_see_horizontal(tree, &trees, outer_index, left_line)
        || can_see_horizontal(tree, &trees, outer_index, right_line)
}

fn can_see_vertical(
    tree: u32,
    trees: &Vec<Vec<u32>>,
    outer_iter: impl Iterator<Item = usize>,
    inner_index: usize,
) -> bool {
    let mut found = false;
    for outer_index in outer_iter {
        if trees[outer_index][inner_index] >= tree {
            found = true;
        }
    }
    !found
}

fn can_see_horizontal(
    tree: u32,
    trees: &Vec<Vec<u32>>,
    outer_index: usize,
    inner_iter: impl Iterator<Item = usize>,
) -> bool {
    let mut found = false;
    for inner_index in inner_iter {
        if trees[outer_index][inner_index] >= tree {
            found = true;
        }
    }
    !found
}

// Part 2

fn calculate_max_scenic_score(trees: &Vec<Vec<u32>>) -> u64 {
    let mut max_score = 0;

    for (outer_index, tree_line) in trees.iter().enumerate() {
        for (inner_index, tree) in tree_line.iter().enumerate() {
            let up_line = 0..outer_index;
            let down_line = (outer_index + 1)..trees.len();
            let left_line = 0..inner_index;
            let right_line = (inner_index + 1)..trees[0].len();

            let up_score = count_can_see_vertical(*tree, &trees, up_line.rev(), inner_index);
            let down_score = count_can_see_vertical(*tree, &trees, down_line, inner_index);
            let left_score = count_can_see_horizontal(*tree, &trees, outer_index, left_line.rev());
            let right_score = count_can_see_horizontal(*tree, &trees, outer_index, right_line);

            let current_score = up_score * down_score * left_score * right_score;

            if current_score > max_score {
                max_score = current_score;
            }
        }
    }

    max_score
}

fn count_can_see_vertical(
    tree: u32,
    trees: &Vec<Vec<u32>>,
    outer_range: impl Iterator<Item = usize>,
    inner_index: usize,
) -> u64 {
    let mut count = 0;

    for outer_index in outer_range {
        count += 1;
        if trees[outer_index][inner_index] >= tree {
            break;
        }
    }

    count
}

fn count_can_see_horizontal(
    tree: u32,
    trees: &Vec<Vec<u32>>,
    outer_index: usize,
    inner_range: impl Iterator<Item = usize>,
) -> u64 {
    let mut count = 0;

    for inner_index in inner_range {
        count += 1;
        if trees[outer_index][inner_index] >= tree {
            break;
        }
    }

    count
}

aoc_main!(Solution; input);
aoc_test!(imp, Solution, input, 1684, 486540);
