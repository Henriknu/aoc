use std::collections::BinaryHeap;

use framework::*;

struct Solution;

impl SolutionProvider for Solution {
    type Output = u64;

    fn part1(input: &String) -> Result<u64> {
        let mut result = 0;
        let mut calories = 0;

        for line in input.lines() {
            match line {
                w if w.is_empty() => {
                    if calories > result {
                        result = calories;
                    }
                    calories = 0;
                }
                x => {
                    let parsed = x.parse::<u64>()?;
                    calories += parsed;
                }
            }
        }

        Ok(result)
    }
    fn part2(input: &String) -> Result<u64> {
        let mut result1 = 0;
        let mut result2 = 0;
        let mut result3 = 0;
        let mut calories = 0;

        for line in input.lines() {
            match line {
                w if w.is_empty() => {
                    if calories > result1 {
                        result3 = result2;
                        result2 = result1;
                        result1 = calories;
                    } else if calories > result2 {
                        result3 = result2;
                        result2 = calories;
                    } else if calories > result3 {
                        result3 = calories;
                    }
                    calories = 0;
                }
                x => {
                    let parsed = x.parse::<u64>()?;
                    calories += parsed;
                }
            }
        }

        Ok(result1 + result2 + result3)
    }
}
struct SolutionFn;

impl SolutionProvider for SolutionFn {
    type Output = u64;

    fn part1(input: &String) -> Result<u64> {
        Ok(input
            .split("\n\n")
            .map(|s| s.lines().map(|ss| ss.parse::<u64>().unwrap()).sum::<u64>())
            .max()
            .unwrap())
    }
    fn part2(input: &String) -> Result<u64> {
        Ok(input
            .split("\n\n")
            .map(|s| s.lines().map(|ss| ss.parse::<u64>().unwrap()).sum::<u64>())
            .collect::<BinaryHeap<u64>>()
            .into_sorted_vec()
            .iter()
            .rev()
            .take(3)
            .sum())
    }
}

aoc_main!(Solution, SolutionFn; input);
aoc_test!(sol, Solution, input, 69528, 206152);
aoc_test!(func, SolutionFn, input, 69528, 206152);
