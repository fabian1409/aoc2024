use std::collections::HashMap;

use aoc_traits::AdventOfCodeDay;

fn possible1(design: &str, patterns: &[&str]) -> bool {
    if design.is_empty() {
        return true;
    }
    for pat in patterns.iter() {
        if pat.len() <= design.len()
            && &design[..pat.len()] == *pat
            && possible1(&design[pat.len()..], patterns)
        {
            return true;
        }
    }
    false
}

fn possible2(design: &str, patterns: &[&str], cache: &mut HashMap<String, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(n) = cache.get(design) {
        return *n;
    }
    let mut count = 0;
    for pat in patterns.iter() {
        if pat.len() <= design.len() && &design[..pat.len()] == *pat {
            count += possible2(&design[pat.len()..], patterns, cache);
        }
    }
    cache.insert(design.to_owned(), count);
    count
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = (Vec<&'a str>, Vec<&'a str>);
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let (patterns, designs) = input.split_once("\n\n").unwrap();
        let patterns = patterns.split(", ").collect();
        let designs = designs.lines().collect();
        (patterns, designs)
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let patterns = &input.0;
        let designs = &input.1;
        designs
            .iter()
            .map(|design| if possible1(design, patterns) { 1 } else { 0 })
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let patterns = &input.0;
        let designs = &input.1;
        designs
            .iter()
            .map(|design| {
                let mut cache = HashMap::new();
                possible2(design, patterns, &mut cache)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    use aoc_traits::AdventOfCodeDay;

    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn test_part1() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 6);
    }

    #[test]
    fn test_part2() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 16);
    }
}
