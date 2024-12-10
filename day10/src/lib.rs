use std::collections::HashSet;

use aoc_traits::AdventOfCodeDay;

fn score1(map: &[Vec<u8>], row: usize, col: usize, heads: &mut HashSet<(usize, usize)>) {
    let cur = map[row][col];
    if cur == 9 {
        heads.insert((row, col));
        return;
    }

    if map[row - 1][col] == cur + 1 {
        score1(map, row - 1, col, heads);
    }
    if map[row + 1][col] == cur + 1 {
        score1(map, row + 1, col, heads);
    }
    if map[row][col - 1] == cur + 1 {
        score1(map, row, col - 1, heads);
    }
    if map[row][col + 1] == cur + 1 {
        score1(map, row, col + 1, heads);
    }
}

fn score2(map: &[Vec<u8>], row: usize, col: usize, heads: &mut usize) {
    let cur = map[row][col];
    if cur == 9 {
        *heads += 1;
        return;
    }

    if map[row - 1][col] == cur + 1 {
        score2(map, row - 1, col, heads);
    }
    if map[row + 1][col] == cur + 1 {
        score2(map, row + 1, col, heads);
    }
    if map[row][col - 1] == cur + 1 {
        score2(map, row, col - 1, heads);
    }
    if map[row][col + 1] == cur + 1 {
        score2(map, row, col + 1, heads);
    }
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<Vec<u8>>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let map = input
            .lines()
            .map(|line| {
                let line = line.chars().map(|c| c as u8 - 48).collect::<Vec<u8>>();
                let mut padded = vec![0];
                padded.extend(line);
                padded.push(0);
                padded
            })
            .collect::<Vec<Vec<u8>>>();
        let mut padded = vec![vec![0; map[0].len()]];
        padded.extend(map);
        padded.push(vec![0; padded[0].len()]);
        padded
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let rows = input.len();
        let cols = input[0].len();

        let mut total_score = 0;
        for row in 1..rows - 1 {
            for col in 1..cols - 1 {
                if input[row][col] == 0 {
                    let mut heads = HashSet::new();
                    score1(input, row, col, &mut heads);
                    total_score += heads.len();
                }
            }
        }

        total_score
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let rows = input.len();
        let cols = input[0].len();

        let mut total_score = 0;
        for row in 1..rows - 1 {
            for col in 1..cols - 1 {
                if input[row][col] == 0 {
                    let mut heads = 0;
                    score2(input, row, col, &mut heads);
                    total_score += heads
                }
            }
        }

        total_score
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    use aoc_traits::AdventOfCodeDay;

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn test_part1() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 36);
    }

    #[test]
    fn test_part2() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 81);
    }
}
