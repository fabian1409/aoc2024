use aoc_traits::AdventOfCodeDay;

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
        let mut stones = input.clone();

        for _ in 0..25 {
            let mut i = 0;
            while i != stones.len() {
                let stone = stones[i];
                let digits = if stone != 0 { stone.ilog10() + 1 } else { 0 };
                if stone == 0 {
                    stones[i] = 1;
                } else if digits % 2 == 0 {
                    let factor = 10u64.pow(digits / 2);
                    let left = stone / factor;
                    let right = stone - left * factor;
                    stones[i] = left;
                    stones.insert(i + 1, right);
                    i += 1;
                } else {
                    stones[i] = stone * 2024;
                }
                i += 1;
            }
        }

        stones.len()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
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
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 0);
    }
}
