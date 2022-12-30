pub extern crate anyhow;
pub use anyhow::Result;

pub trait SolutionProvider {
    type Output;

    fn part1(input: &String) -> Result<Self::Output>;
    fn part2(input: &String) -> Result<Self::Output>;
}

#[macro_export]
macro_rules! aoc_main {
    ($($solution:ty),+; $input_path: ident) => {
        fn main() -> $crate::anyhow::Result<()> {
            let input = std::fs::read_to_string(format!("data/{}.txt", stringify!($input_path)))?;
            $(
                let result1 = <$solution>::part1(&input).unwrap();
                let result2 = <$solution>::part2(&input).unwrap();

                println!("Solution: {}, Part1: {result1}, Part2: {result2}", stringify!($solution));

            )+

            Ok(())
        }
    };
}

#[macro_export]
macro_rules! aoc_test {
    ($test_m:ident, $solution:ty, $input_path: ident, $ans_1: expr, $ans_2: expr) => {
        #[cfg(test)]
        mod $test_m {
            use super::*;

            #[test]
            fn part1() {
                let input =
                    std::fs::read_to_string(format!("data/{}.txt", stringify!($input_path)))
                        .unwrap();
                assert_eq!(<$solution>::part1(&input).unwrap(), $ans_1);
            }

            #[test]
            fn part2() {
                let input =
                    std::fs::read_to_string(format!("data/{}.txt", stringify!($input_path)))
                        .unwrap();
                assert_eq!(<$solution>::part2(&input).unwrap(), $ans_2);
            }
        }
    };
}
