use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, Copy)]
enum Op1 {
    Add,
    Mul,
}

fn satisfiable1(res: u64, mut acc: u64, terms: &[u64], op: Op1) -> bool {
    if acc == res && terms.is_empty() {
        true
    } else if acc <= res && !terms.is_empty() {
        match op {
            Op1::Add => acc += terms[0],
            Op1::Mul => acc *= terms[0],
        };
        satisfiable1(res, acc, &terms[1..], Op1::Add)
            || satisfiable1(res, acc, &terms[1..], Op1::Mul)
    } else {
        false
    }
}

#[derive(Debug, Clone, Copy)]
enum Op2 {
    Add,
    Mul,
    Concat,
}

fn satisfiable2(res: u64, mut acc: u64, terms: &[u64], op: Op2) -> bool {
    if acc == res && terms.is_empty() {
        true
    } else if acc <= res && !terms.is_empty() {
        match op {
            Op2::Add => acc += terms[0],
            Op2::Mul => acc *= terms[0],
            Op2::Concat => {
                let term = terms[0];
                let digits = term.ilog10() + 1;
                acc *= 10u64.pow(digits);
                acc += term;
            }
        };
        satisfiable2(res, acc, &terms[1..], Op2::Add)
            || satisfiable2(res, acc, &terms[1..], Op2::Mul)
            || satisfiable2(res, acc, &terms[1..], Op2::Concat)
    } else {
        false
    }
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<(u64, Vec<u64>)>;
    type Part1Output = u64;
    type Part2Output = u64;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input
            .lines()
            .map(|line| {
                let (res, terms) = line.split_once(": ").unwrap();
                (
                    res.parse().unwrap(),
                    terms.split(' ').map(|t| t.parse().unwrap()).collect(),
                )
            })
            .collect()
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input
            .iter()
            .map(|(res, terms)| {
                if satisfiable1(*res, terms[0], &terms[1..], Op1::Add)
                    || satisfiable1(*res, terms[0], &terms[1..], Op1::Mul)
                {
                    *res
                } else {
                    0
                }
            })
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input
            .iter()
            .map(|(res, terms)| {
                if satisfiable2(*res, terms[0], &terms[1..], Op2::Add)
                    || satisfiable2(*res, terms[0], &terms[1..], Op2::Mul)
                    || satisfiable2(*res, terms[0], &terms[1..], Op2::Concat)
                {
                    *res
                } else {
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

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_part1() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 3749);
    }

    #[test]
    fn test_part2() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 11387);
    }
}
