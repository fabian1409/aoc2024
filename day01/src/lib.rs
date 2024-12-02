use aoc_traits::AdventOfCodeDay;

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = (Vec<u32>, Vec<u32>);
    type Part1Output = u32;
    type Part2Output = u32;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let len = input.lines().count();
        let mut left = Vec::with_capacity(len);
        let mut right = Vec::with_capacity(len);
        for line in input.lines() {
            let (l, r) = line.split_once("   ").expect("<num>   <num>");
            left.push(l.parse().expect("valid number"));
            right.push(r.parse().expect("valid number"));
        }
        (left, right)
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let mut left = input.0.clone();
        let mut right = input.1.clone();
        left.sort();
        right.sort();
        left.into_iter()
            .zip(right)
            .map(|(l, r)| l.abs_diff(r))
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let max0 = input.0.iter().max().expect("max exists");
        let max1 = input.1.iter().max().expect("max exists");
        let mut counts = vec![0u32; *max0.max(max1) as usize + 1];
        for num in input.1.iter() {
            counts[*num as usize] += 1;
        }
        input.0.iter().map(|num| num * counts[*num as usize]).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    use aoc_traits::AdventOfCodeDay;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_part1() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 11);
    }

    #[test]
    fn test_part2() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 31);
    }
}
