use aoc_traits::AdventOfCodeDay;
use memoize::memoize;

#[memoize]
fn apply(stone: u64, depth: usize) -> usize {
    let mut count = 0;
    if depth == 0 {
        return count;
    }

    let digits = if stone != 0 { stone.ilog10() + 1 } else { 0 };
    if stone == 0 {
        count += apply(stone + 1, depth - 1);
    } else if digits % 2 == 0 {
        let factor = 10u64.pow(digits / 2);
        let left = stone / factor;
        let right = stone - left * factor;
        count += 1;
        count += apply(left, depth - 1);
        count += apply(right, depth - 1);
    } else {
        count += apply(stone * 2024, depth - 1);
    }

    count
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<u64>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input.split(' ').map(|s| s.parse().unwrap()).collect()
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let mut count = input.len();
        for stone in input {
            count += apply(*stone, 25);
        }
        count
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let mut count = input.len();
        for stone in input {
            count += apply(*stone, 75);
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::Solver;
    use aoc_traits::AdventOfCodeDay;

    const INPUT: &str = "77 515 6779622 6 91370 959685 0 9861";

    #[test]
    fn test_part1() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 187738);
    }

    #[test]
    fn test_part2() {
        let start = Instant::now();
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 223767210249237);
        println!("took {}", start.elapsed().as_micros());
    }
}
