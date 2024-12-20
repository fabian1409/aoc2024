use std::collections::HashMap;

use aoc_traits::AdventOfCodeDay;

fn possible<'a>(design: &'a str, patterns: &[&str], cache: &mut HashMap<&'a str, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(n) = cache.get(design) {
        return *n;
    }
    let mut count = 0;
    for pat in patterns.iter() {
        if let Some(rest) = design.strip_prefix(pat) {
            let n = possible(rest, patterns, cache);
            cache.insert(rest, n);
            count += n;
        }
    }
    cache.insert(design, count);
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
        let mut cache = HashMap::new();
        designs
            .iter()
            .filter(|design| possible(design, patterns, &mut cache) > 0)
            .count()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let patterns = &input.0;
        let designs = &input.1;
        let mut cache = HashMap::new();
        designs
            .iter()
            .map(|design| possible(design, patterns, &mut cache))
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
