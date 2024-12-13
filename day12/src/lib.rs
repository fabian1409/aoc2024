use std::collections::HashSet;

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    pub fn offset(&self, row: usize, col: usize) -> (usize, usize) {
        match self {
            Dir::Left => (row, col - 1),
            Dir::Right => (row, col + 1),
            Dir::Up => (row - 1, col),
            Dir::Down => (row + 1, col),
        }
    }
}

fn area_perimiter(
    input: &[Vec<u8>],
    row: usize,
    col: usize,
    plant: u8,
    visted: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let mut area = 1;
    let mut perimiter = 0;

    visted.insert((row, col));

    for dir in [Dir::Left, Dir::Right, Dir::Up, Dir::Down] {
        let (row, col) = dir.offset(row, col);

        if input[row][col] != plant {
            perimiter += 1;
        }

        if !visted.contains(&(row, col)) && input[row][col] == plant {
            let (a, p) = area_perimiter(input, row, col, input[row][col], visted);
            area += a;
            perimiter += p;
        }
    }

    (area, perimiter)
}

fn area_region(
    input: &[Vec<u8>],
    row: usize,
    col: usize,
    plant: u8,
    region: &mut [Vec<bool>],
    visted: &mut HashSet<(usize, usize)>,
) -> usize {
    let mut area = 1;

    visted.insert((row, col));
    region[row][col] = true;

    for dir in [Dir::Left, Dir::Right, Dir::Up, Dir::Down] {
        let (row, col) = dir.offset(row, col);
        if !visted.contains(&(row, col)) && input[row][col] == plant {
            area += area_region(input, row, col, input[row][col], region, visted);
        }
    }

    area
}

fn sides(input: &mut [Vec<bool>]) -> usize {
    let rows = input.len();
    let cols = input[0].len();

    let mut top = 0;
    let mut bot = 0;
    let mut right = 0;
    let mut left = 0;

    let mut in_top_side = false;
    let mut in_bot_side = false;

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if in_top_side {
                if !input[row][col + 1] || input[row - 1][col + 1] {
                    // println!("stopped side {row} {col}");
                    in_top_side = false;
                }
            } else if !input[row - 1][col] && input[row][col] {
                // println!("started side {row} {col}");
                top += 1;
                in_top_side = true;
                if !input[row][col + 1] || input[row - 1][col + 1] {
                    // println!("stopped side {row} {col}");
                    in_top_side = false;
                }
            }

            if in_bot_side {
                if !input[row][col + 1] || input[row + 1][col + 1] {
                    // println!("stopped side {row} {col}");
                    in_bot_side = false;
                }
            } else if !input[row + 1][col] && input[row][col] {
                // println!("started side {row} {col}");
                bot += 1;
                in_bot_side = true;
                if !input[row][col + 1] || input[row + 1][col + 1] {
                    // println!("stopped side {row} {col}");
                    in_bot_side = false;
                }
            }
        }
    }

    let mut in_right_side = false;
    let mut in_left_side = false;

    for col in 1..cols - 1 {
        for row in 1..rows - 1 {
            if in_right_side {
                if !input[row + 1][col] || input[row + 1][col + 1] {
                    // println!("stopped side {row} {col}");
                    in_right_side = false;
                }
            } else if !input[row][col + 1] && input[row][col] {
                // println!("started side {row} {col}");
                right += 1;
                in_right_side = true;
                if !input[row + 1][col] || input[row + 1][col + 1] {
                    // println!("stopped side {row} {col}");
                    in_right_side = false;
                }
            }

            if in_left_side {
                if !input[row + 1][col] || input[row + 1][col - 1] {
                    // println!("stopped side {row} {col}");
                    in_left_side = false;
                }
            } else if !input[row][col - 1] && input[row][col] {
                // println!("started side {row} {col}");
                left += 1;
                in_left_side = true;
                if !input[row + 1][col] || input[row + 1][col - 1] {
                    // println!("stopped side {row} {col}");
                    in_left_side = false;
                }
            }
        }
    }

    top + bot + left + right
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
                    let (area, perimiter) =
                        area_perimiter(input, row, col, input[row][col], &mut visited);
                    price += area * perimiter;
                }
            }
        }

        price
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let rows = input.len();
        let cols = input[0].len();
        let mut visited = HashSet::new();
        let mut price = 0;

        for row in 1..rows - 1 {
            for col in 1..cols - 1 {
                if !visited.contains(&(row, col)) {
                    let mut region = vec![vec![false; cols]; rows];
                    let area =
                        area_region(input, row, col, input[row][col], &mut region, &mut visited);
                    let sides = sides(&mut region);
                    price += area * sides;
                }
            }
        }

        price
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
        assert_eq!(Solver::solve_part2(&parsed), 80);
    }
}
