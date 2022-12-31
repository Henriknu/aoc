use framework::*;

struct Solution;

impl SolutionProvider for Solution {
    type Output = u64;

    fn part1(input: &String) -> Result<u64> {
        let mut score = 0;

        for assignments in input.lines() {
            let mut split = assignments.split(',');
            let (assignment1, assignment2) = (
                Assignment::parse(split.next().unwrap())?,
                Assignment::parse(split.next().unwrap())?,
            );

            if check_if_overlap_fully(assignment1, assignment2) {
                score += 1;
            }
        }

        Ok(score)
    }
    fn part2(input: &String) -> Result<u64> {
        let mut score = 0;

        for assignments in input.lines() {
            let mut split = assignments.split(',');
            let (assignment1, assignment2) = (
                Assignment::parse(split.next().unwrap())?,
                Assignment::parse(split.next().unwrap())?,
            );

            if check_if_overlap_partially(assignment1, assignment2) {
                score += 1;
            }
        }

        Ok(score)
    }
}

fn check_if_overlap_fully(assignment1: Assignment, assignment2: Assignment) -> bool {
    (assignment1.start <= assignment2.start && assignment1.end >= assignment2.end)
        || (assignment2.start <= assignment1.start && assignment2.end >= assignment1.end)
}

fn check_if_overlap_partially(assignment1: Assignment, assignment2: Assignment) -> bool {
    (assignment1.start >= assignment2.start && assignment1.start <= assignment2.end)
        || (assignment2.start >= assignment1.start && assignment2.start <= assignment1.end)
}

#[derive(Clone, Copy, Debug)]
struct Assignment {
    start: u8,
    end: u8,
}

impl Assignment {
    fn parse(assignment: &str) -> Result<Assignment> {
        let mut split = assignment.split('-');

        let start = split.next().unwrap().parse::<u8>()?;
        let end = split.next().unwrap().parse::<u8>()?;

        Ok(Assignment { start, end })
    }
}

aoc_main!(Solution; input);
aoc_test!(imp, Solution, input, 13052, 13693);
