use aoc_traits::AdventOfCodeDay;
use itertools::Itertools;

fn increasing(pair: (&u32, &u32)) -> bool {
    (1..=3).contains(&(pair.0 - pair.1))
}

fn decreasing(pair: (&u32, &u32)) -> bool {
    (-3..=-1).contains(&(*pair.0 as i32 - *pair.1 as i32))
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<Vec<u32>>;
    type Part1Output = u32;
    type Part2Output = u32;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|x| x.parse().expect("valid number"))
                    .collect()
            })
            .collect()
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input
            .iter()
            .map(|report| {
                if report.iter().tuple_windows().all(increasing)
                    || report.iter().tuple_windows().all(decreasing)
                {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input
            .iter()
            .map(|report| {
                // TODO we could check wich idx was invalid and only check form idx - 1.. below
                if report.iter().tuple_windows().all(increasing)
                    || report.iter().tuple_windows().all(decreasing)
                {
                    1
                } else {
                    for remove in 0..report.len() {
                        if report
                            .iter()
                            .enumerate()
                            .filter_map(|(i, e)| if i == remove { None } else { Some(e) })
                            .tuple_windows()
                            .all(increasing)
                            || report
                                .iter()
                                .enumerate()
                                .filter_map(|(i, e)| if i == remove { None } else { Some(e) })
                                .tuple_windows()
                                .all(decreasing)
                        {
                            return 1;
                        }
                    }
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    use aoc_traits::AdventOfCodeDay;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_part1() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 2);
    }

    #[test]
    fn test_part2() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 4);
    }
}
