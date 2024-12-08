use std::collections::HashMap;

use aoc_traits::AdventOfCodeDay;
use itertools::Itertools;

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = (usize, usize, HashMap<u8, Vec<(i32, i32)>>);
    type Part1Output = u32;
    type Part2Output = u32;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let mut rows = 0;
        let mut cols = 0;
        let mut antennas = HashMap::<u8, Vec<_>>::new();
        for (row, line) in input.lines().enumerate() {
            rows += 1;
            let line = line.as_bytes();
            cols = line.len();
            for (col, c) in line.iter().enumerate() {
                if *c != b'.' {
                    antennas
                        .entry(*c)
                        .or_insert(Vec::new())
                        .push((row as i32, col as i32));
                }
            }
        }
        (rows, cols, antennas)
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let mut antinodes = vec![vec![false; input.0]; input.1];
        for (_antenna, points) in input.2.iter() {
            for ab in points.iter().combinations(2) {
                let a = ab[0];
                let b = ab[1];
                let v = (b.0 - a.0, b.1 - a.1);

                let antinode0 = (b.0 + v.0, b.1 + v.1);
                if antinode0.0 >= 0
                    && antinode0.0 < input.0 as i32
                    && antinode0.1 >= 0
                    && antinode0.1 < input.1 as i32
                {
                    antinodes[antinode0.0 as usize][antinode0.1 as usize] = true;
                }

                let antinode1 = (a.0 - v.0, a.1 - v.1);
                if antinode1.0 >= 0
                    && antinode1.0 < input.0 as i32
                    && antinode1.1 >= 0
                    && antinode1.1 < input.1 as i32
                {
                    antinodes[antinode1.0 as usize][antinode1.1 as usize] = true;
                }
            }
        }

        antinodes
            .iter()
            .flatten()
            .map(|n| if *n { 1 } else { 0 })
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let mut antinodes = vec![vec![false; input.0]; input.1];
        for (_antenna, points) in input.2.iter() {
            for ab in points.iter().combinations(2) {
                let a = ab[0];
                let b = ab[1];
                let v = (b.0 - a.0, b.1 - a.1);

                let mut antinode0 = *b;
                loop {
                    if antinode0.0 >= 0
                        && antinode0.0 < input.0 as i32
                        && antinode0.1 >= 0
                        && antinode0.1 < input.1 as i32
                    {
                        antinodes[antinode0.0 as usize][antinode0.1 as usize] = true;
                    } else {
                        break;
                    }
                    antinode0 = (antinode0.0 + v.0, antinode0.1 + v.1);
                }

                let mut antinode1 = *a;
                loop {
                    if antinode1.0 >= 0
                        && antinode1.0 < input.0 as i32
                        && antinode1.1 >= 0
                        && antinode1.1 < input.1 as i32
                    {
                        antinodes[antinode1.0 as usize][antinode1.1 as usize] = true;
                    } else {
                        break;
                    }
                    antinode1 = (antinode1.0 - v.0, antinode1.1 - v.1);
                }
            }
        }

        antinodes
            .iter()
            .flatten()
            .map(|n| if *n { 1 } else { 0 })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    use aoc_traits::AdventOfCodeDay;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_part1() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 14);
    }

    #[test]
    fn test_part2() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 34);
    }
}
