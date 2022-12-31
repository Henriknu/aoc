use framework::{anyhow::Context, *};

struct Solution;

impl SolutionProvider for Solution {
    type Output = u64;

    fn part1(input: &String) -> Result<u64> {
        let mut count = None;

        let mut start = 0;
        let mut end = 4;

        while end <= input.len() {
            if !find_duplicate(input, start, end) {
                count = Some(end as u64);
                break;
            }

            start += 1;
            end += 1;
        }

        count.context("Did not find marker")
    }
    fn part2(input: &String) -> Result<u64> {
        let mut count = None;

        let mut start = 0;
        let mut end = 14;

        while end <= input.len() {
            if !find_duplicate(input, start, end) {
                count = Some(end as u64);
                break;
            }

            start += 1;
            end += 1;
        }

        count.context("Did not find marker")
    }
}

fn find_duplicate(haystack: &str, start: usize, end: usize) -> bool {
    for (index, char) in haystack[start..end].char_indices() {
        if haystack[(start + index + 1)..end].contains(char) {
            return true;
        }
    }
    false
}

aoc_main!(Solution; input);
aoc_test!(imp, Solution, input, 1140, 3495);
