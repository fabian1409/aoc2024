use std::collections::HashSet;

use aoc_traits::AdventOfCodeDay;

fn calc(
    input: &[Vec<u8>],
    row: usize,
    col: usize,
    plant: u8,
    visted: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let mut area = 0;
    let mut perimiter = 0;
    let left = (row, col - 1);
    let right = (row, col + 1);
    let up = (row - 1, col);
    let down = (row + 1, col);

    visted.insert((row, col));
    area += 1;

    if input[left.0][left.1] != plant {
        perimiter += 1;
    }
    if input[right.0][right.1] != plant {
        perimiter += 1;
    }
    if input[up.0][up.1] != plant {
        perimiter += 1;
    }
    if input[down.0][down.1] != plant {
        perimiter += 1;
    }

    if !visted.contains(&left) && input[left.0][left.1] == plant {
        let (a, p) = calc(input, left.0, left.1, input[left.0][left.1], visted);
        area += a;
        perimiter += p;
    }
    if !visted.contains(&right) && input[right.0][right.1] == plant {
        let (a, p) = calc(input, right.0, right.1, input[right.0][right.1], visted);
        area += a;
        perimiter += p;
    }
    if !visted.contains(&up) && input[up.0][up.1] == plant {
        let (a, p) = calc(input, up.0, up.1, input[up.0][up.1], visted);
        area += a;
        perimiter += p;
    }
    if !visted.contains(&down) && input[down.0][down.1] == plant {
        let (a, p) = calc(input, down.0, down.1, input[down.0][down.1], visted);
        area += a;
        perimiter += p;
    }

    (area, perimiter)
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<Vec<u8>>;
    type Part1Output = usize;
    type Part2Output = u32;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let map = input
            .lines()
            .map(|line| {
                let line = line.chars().map(|c| c as u8).collect::<Vec<u8>>();
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
        let mut visited = HashSet::new();
        let mut price = 0;

        for row in 1..rows - 1 {
            for col in 1..cols - 1 {
                if !visited.contains(&(row, col)) {
                    let (area, perimiter) = calc(input, row, col, input[row][col], &mut visited);
                    price += area * perimiter;
                }
            }
        }

        price
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    use aoc_traits::AdventOfCodeDay;

    const INPUT: &str = "AAAA
BBCD
BBCC
EEEC
";

    #[test]
    fn test_part1() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 140);
    }

    #[test]
    fn test_part2() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 0);
    }
}
