use std::collections::HashSet;

use aoc_traits::AdventOfCodeDay;
use itertools::Itertools;

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = (HashSet<(u32, u32)>, Vec<Vec<u32>>);
    type Part1Output = u32;
    type Part2Output = u32;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let (rules, updates) = input.split_once("\n\n").unwrap();
        let rules = rules
            .lines()
            .map(|line| {
                let (a, b) = line.split_once('|').unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect();
        let updates = updates
            .lines()
            .map(|line| line.split(',').map(|p| p.parse().unwrap()).collect())
            .collect();
        (rules, updates)
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let rules = &input.0;
        let updates = &input.1;

        updates
            .iter()
            .map(|line| {
                if line
                    .iter()
                    .tuple_windows::<(_, _)>()
                    .all(|pair| rules.contains(&(*pair.0, *pair.1)))
                {
                    line[line.len() / 2]
                } else {
                    0
                }
            })
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    use aoc_traits::AdventOfCodeDay;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_part1() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 143);
    }

    #[test]
    fn test_part2() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 31);
    }
}
