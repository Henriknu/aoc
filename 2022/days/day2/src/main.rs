use std::collections::HashMap;

use framework::*;
use once_cell::sync::OnceCell;

type BeatsMap = HashMap<char, HashMap<char, u64>>;

static BEATS1: OnceCell<BeatsMap> = OnceCell::new();
static BEATS2: OnceCell<BeatsMap> = OnceCell::new();

struct Solution;

impl SolutionProvider for Solution {
    type Output = u64;

    fn part1(input: &String) -> Result<u64> {
        let mut total_score = 0;

        for line in input.lines() {
            let opponent_move = line.chars().nth(0).unwrap();
            let player_move = line.chars().nth(2).unwrap();

            total_score += get_beats_part_1()[&opponent_move][&player_move];
        }

        Ok(total_score)
    }
    fn part2(input: &String) -> Result<u64> {
        let mut total_score = 0;

        for line in input.lines() {
            let opponent_move = line.chars().nth(0).unwrap();
            let player_move = line.chars().nth(2).unwrap();

            total_score += get_beats_part_2()[&opponent_move][&player_move];
        }

        Ok(total_score)
    }
}

fn get_beats_part_1() -> &'static BeatsMap {
    BEATS1.get_or_init(|| {
        let mut map = BeatsMap::new();

        let mut rock_map = HashMap::new();
        rock_map.insert('X', 3 + 1);
        rock_map.insert('Y', 6 + 2);
        rock_map.insert('Z', 0 + 3);

        let mut paper_map = HashMap::new();
        paper_map.insert('X', 0 + 1);
        paper_map.insert('Y', 3 + 2);
        paper_map.insert('Z', 6 + 3);

        let mut scissor_map = HashMap::new();
        scissor_map.insert('X', 6 + 1);
        scissor_map.insert('Y', 0 + 2);
        scissor_map.insert('Z', 3 + 3);

        map.insert('A', rock_map);
        map.insert('B', paper_map);
        map.insert('C', scissor_map);

        map
    })
}

fn get_beats_part_2() -> &'static BeatsMap {
    BEATS2.get_or_init(|| {
        let mut map = BeatsMap::new();

        let mut rock_map = HashMap::new();
        rock_map.insert('X', 0 + 3);
        rock_map.insert('Y', 3 + 1);
        rock_map.insert('Z', 6 + 2);

        let mut paper_map = HashMap::new();
        paper_map.insert('X', 0 + 1);
        paper_map.insert('Y', 3 + 2);
        paper_map.insert('Z', 6 + 3);

        let mut scissor_map = HashMap::new();
        scissor_map.insert('X', 0 + 2);
        scissor_map.insert('Y', 3 + 3);
        scissor_map.insert('Z', 6 + 1);

        map.insert('A', rock_map);
        map.insert('B', paper_map);
        map.insert('C', scissor_map);

        map
    })
}

aoc_main!(Solution; input);
aoc_test!(imp, Solution, input, 13052, 13693);
