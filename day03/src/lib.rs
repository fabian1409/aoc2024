use aoc_traits::AdventOfCodeDay;
use regex::Regex;

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = &'a str;
    type Part1Output = u32;
    type Part2Output = u32;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        re.captures_iter(input)
            .map(|c| c.extract())
            .map(|(_, [a, b])| {
                a.parse::<u32>().expect("valid number") * b.parse::<u32>().expect("valid number")
            })
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let re = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)|(?<do>do\(\))|(?<dont>don't\(\))")
            .unwrap();
        let mut enabled = true;
        re.captures_iter(input)
            .filter_map(|c| {
                if c.name("do").is_some() {
                    enabled = true;
                } else if c.name("dont").is_some() {
                    enabled = false;
                } else {
                    let a = c.name("a").unwrap();
                    let b = c.name("b").unwrap();
                    if enabled {
                        return Some(
                            a.as_str().parse::<u32>().expect("valid number")
                                * b.as_str().parse::<u32>().expect("valid number"),
                        );
                    }
                }
                None
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    use aoc_traits::AdventOfCodeDay;

    const INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 161);
    }

    #[test]
    fn test_part2() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 48);
    }
}
