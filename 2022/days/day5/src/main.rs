use std::collections::VecDeque;

use framework::*;

type Stack = VecDeque<char>;
type Stacks = Vec<Stack>;

struct Solution;

impl SolutionProvider for Solution {
    type Output = String;

    fn part1(input: &String) -> Result<String> {
        let mut split = input.split("\n\n");
        let crates = split.next().unwrap();
        let instructions = split.next().unwrap();

        let mut stacks = parse_stacks(crates);
        let instructions = parse_instructions(instructions);

        execute_instructions_9000(&mut stacks, &instructions);
        let message = compute_message(&stacks);

        Ok(message)
    }
    fn part2(input: &String) -> Result<String> {
        let mut split = input.split("\n\n");
        let crates = split.next().unwrap();
        let instructions = split.next().unwrap();

        let mut stacks = parse_stacks(crates);
        let instructions = parse_instructions(instructions);

        execute_instructions_9001(&mut stacks, &instructions);
        let message = compute_message(&stacks);

        Ok(message)
    }
}

fn compute_message(stacks: &Stacks) -> String {
    stacks.iter().filter_map(|v| v.front()).collect()
}

fn execute_instructions_9000(stacks: &mut Stacks, instructions: &[Instruction]) {
    for instruction in instructions {
        let stack_from = &mut stacks[instruction.from - 1];
        let crates_from = stack_from.drain(..instruction.quantity).collect::<Vec<_>>();

        let stack_to = &mut stacks[instruction.to - 1];

        for crate_ in crates_from.into_iter() {
            stack_to.push_front(crate_)
        }
    }
}

fn execute_instructions_9001(stacks: &mut Stacks, instructions: &[Instruction]) {
    for instruction in instructions {
        dbg!(&stacks);

        let stack_from = &mut stacks[instruction.from - 1];
        let crates_from = stack_from
            .drain(..instruction.quantity)
            .collect::<VecDeque<_>>();

        let stack_to = &mut stacks[instruction.to - 1];

        for crate_ in crates_from.into_iter().rev() {
            stack_to.push_front(crate_)
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    let mut result = Vec::new();

    for line in instructions.lines() {
        let mut split = line.split_whitespace();

        let quantity = split.nth(1).unwrap().parse().unwrap();
        let from = split.nth(1).unwrap().parse().unwrap();
        let to = split.nth(1).unwrap().parse().unwrap();

        result.push(Instruction { quantity, from, to })
    }

    result
}

fn parse_stacks(crates: &str) -> Stacks {
    let mut lines = crates.lines().peekable();

    let num_stacks = (lines.peek().unwrap().len() + 1) / 4;

    let mut stacks = vec![VecDeque::new(); num_stacks];

    while let Some(line) = lines.next() {
        if lines.peek().is_none() {
            // last line is stack indices, no crates to add.
            break;
        }

        let mut chars = line.chars();
        let mut index = 0;

        while let Some(maybe_crate) = chars.nth(1) {
            if !maybe_crate.is_whitespace() {
                stacks[index].push_back(maybe_crate)
            }
            index += 1;
            chars.nth(1);
        }
    }

    stacks
}

aoc_main!(Solution; input);
aoc_test!(
    imp,
    Solution,
    input,
    "SHQWSRBDL".to_owned(),
    "CDTQZHBRS".to_owned()
);
