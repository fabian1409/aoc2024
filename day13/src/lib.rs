use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, Copy)]
pub struct Claw {
    pub a: (usize, usize),
    pub b: (usize, usize),
    pub c: (usize, usize),
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<Claw>;
    type Part1Output = usize;
    type Part2Output = usize;

    // Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400
    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input
            .split("\n\n")
            .map(|claw| {
                let vals = claw
                    .lines()
                    .map(|line| {
                        let x_pos = line.chars().position(|c| c == 'X').unwrap();
                        let y_pos = line.chars().position(|c| c == 'Y').unwrap();
                        let comma_pos = line.chars().position(|c| c == ',').unwrap();
                        let v0 = line[x_pos + 2..comma_pos].parse().unwrap();
                        let v1 = line[y_pos + 2..].parse().unwrap();
                        (v0, v1)
                    })
                    .collect::<Vec<_>>();
                Claw {
                    a: vals[0],
                    b: vals[1],
                    c: vals[2],
                }
            })
            .collect()
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input
            .iter()
            .map(|claw| {
                let a0 = claw.a.0 as f64;
                let a1 = claw.a.1 as f64;
                let b0 = claw.b.0 as f64;
                let b1 = claw.b.1 as f64;
                let c0 = claw.c.0 as f64;
                let c1 = claw.c.1 as f64;

                let x0 = (b0 * -c1 - b1 * -c0) / (a0 * b1 - a1 * b0);
                let y0 = (-c0 * a1 - -c1 * a0) / (a0 * b1 - a1 * b0);

                if x0.floor() == x0 && y0.floor() == y0 {
                    x0 as usize * 3 + y0 as usize
                } else {
                    0
                }
            })
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input
            .iter()
            .map(|claw| {
                let a0 = claw.a.0 as f64;
                let a1 = claw.a.1 as f64;
                let b0 = claw.b.0 as f64;
                let b1 = claw.b.1 as f64;
                let c0 = (claw.c.0 + 10000000000000) as f64;
                let c1 = (claw.c.1 + 10000000000000) as f64;

                let x0 = (b0 * -c1 - b1 * -c0) / (a0 * b1 - a1 * b0);
                let y0 = (-c0 * a1 - -c1 * a0) / (a0 * b1 - a1 * b0);

                if x0.floor() == x0 && y0.floor() == y0 {
                    x0 as usize * 3 + y0 as usize
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

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test_part1() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 480);
    }

    #[test]
    fn test_part2() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 875318608908);
    }
}
