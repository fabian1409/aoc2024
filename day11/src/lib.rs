use aoc_traits::AdventOfCodeDay;
use memoize::memoize;

#[memoize]
fn apply(stone: u64, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    if stone == 0 {
        apply(stone + 1, depth - 1)
    } else if (stone.ilog10() + 1) % 2 == 0 {
        let factor = 10u64.pow((stone.ilog10() + 1) / 2);
        let left = stone / factor;
        let right = stone % factor;
        apply(left, depth - 1) + apply(right, depth - 1)
    } else {
        apply(stone * 2024, depth - 1)
    }
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
        let res = input.iter().map(|stone| apply(*stone, 25)).sum();
        memoized_flush_apply();
        res
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let res = input.iter().map(|stone| apply(*stone, 75)).sum();
        memoized_flush_apply();
        res
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
