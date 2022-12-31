use framework::*;

struct Solution;

impl SolutionProvider for Solution {
    type Output = u64;

    fn part1(input: &String) -> Result<u64> {
        let mut score = 0;

        for rucksack in input.lines() {
            let (first, second) = rucksack.split_at(rucksack.len() / 2);

            let mut errors = Vec::new();

            for item in first.chars() {
                if !errors.contains(&item) && second.contains(item) {
                    errors.push(item);
                }
            }

            score += extract_priority_score(errors);
        }

        Ok(score)
    }
    fn part2(input: &String) -> Result<u64> {
        let mut score = 0;

        let mut rucksacks = input.lines();

        'outer: while let (Some(sack1), Some(sack2), Some(sack3)) =
            (rucksacks.next(), rucksacks.next(), rucksacks.next())
        {
            for item in sack1.chars() {
                if sack2.contains(item) && sack3.contains(item) {
                    dbg!(item);
                    score += char_to_codepoint(item);
                    continue 'outer;
                }
            }
            unreachable!()
        }

        Ok(score)
    }
}

fn extract_priority_score(errors: Vec<char>) -> u64 {
    errors.iter().map(|c| char_to_codepoint(*c)).sum::<u64>()
}

fn char_to_codepoint(c: char) -> u64 {
    let offset = if c.is_uppercase() { 38 } else { 96 };
    (c as u32 - offset) as u64
}

aoc_main!(Solution; input);
aoc_test!(imp, Solution, input, 7917, 2585);
