use aoc_traits::AdventOfCodeDay;

const X: u8 = 88;
const M: u8 = 77;
const A: u8 = 65;
const S: u8 = 83;

const XMAS: [u8; 4] = [X, M, A, S];
const SAMX: [u8; 4] = [S, A, M, X];
const MAS: [u8; 3] = [M, A, S];
const SAM: [u8; 3] = [S, A, M];

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = (Vec<u8>, usize, usize);
    type Part1Output = u32;
    type Part2Output = u32;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let mut rows = 0;
        let cols = input.lines().next().unwrap().len();
        let data = input
            .lines()
            .flat_map(|line| {
                rows += 1;
                line.chars().map(|c| c.to_ascii_uppercase() as u8)
            })
            .collect();
        (data, rows, cols)
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let data = &input.0;
        let rows = input.1;
        let cols = input.2;

        let index = |row: usize, col: usize| -> usize { col + cols * row };

        let mut count = 0;

        for row in 0..rows {
            for col in 0..cols {
                // TODO merge duplicate array accesses
                // only check if cur is X or S, or if right to left diag is X or S
                if data[index(row, col)] == X
                    || data[index(row, col)] == S
                    || (col < cols - 3
                        && (data[index(row, col + 3)] == X || data[index(row, col + 3)] == S))
                {
                    if row < rows - 3 && col < cols - 3 {
                        let h0 = data[index(row, col)];
                        let h1 = data[index(row, col + 1)];
                        let h2 = data[index(row, col + 2)];
                        let h3 = data[index(row, col + 3)];
                        let v0 = data[index(row, col)];
                        let v1 = data[index(row + 1, col)];
                        let v2 = data[index(row + 2, col)];
                        let v3 = data[index(row + 3, col)];
                        let da0 = data[index(row, col)];
                        let da1 = data[index(row + 1, col + 1)];
                        let da2 = data[index(row + 2, col + 2)];
                        let da3 = data[index(row + 3, col + 3)];
                        let db0 = data[index(row, col + 3)];
                        let db1 = data[index(row + 1, col + 2)];
                        let db2 = data[index(row + 2, col + 1)];
                        let db3 = data[index(row + 3, col)];

                        let h = [h0, h1, h2, h3];
                        let v = [v0, v1, v2, v3];
                        let da = [da0, da1, da2, da3];
                        let db = [db0, db1, db2, db3];

                        if h == XMAS
                            || h == SAMX
                            || v == XMAS
                            || v == SAMX
                            || da == XMAS
                            || da == SAMX
                            || db == XMAS
                            || db == SAMX
                        {
                            count += 1;
                        }
                    } else if row < rows - 3 {
                        let v0 = data[index(row, col)];
                        let v1 = data[index(row + 1, col)];
                        let v2 = data[index(row + 2, col)];
                        let v3 = data[index(row + 3, col)];

                        let v = [v0, v1, v2, v3];

                        if v == XMAS || v == SAMX {
                            count += 1;
                        }
                    } else if col < cols - 3 {
                        let h0 = data[index(row, col)];
                        let h1 = data[index(row, col + 1)];
                        let h2 = data[index(row, col + 2)];
                        let h3 = data[index(row, col + 3)];

                        let h = [h0, h1, h2, h3];

                        if h == XMAS || h == SAMX {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let data = &input.0;
        let rows = input.1;
        let cols = input.2;

        let index = |row: usize, col: usize| -> usize { col + cols * row };

        let mut count = 0;

        for row in 1..rows - 1 {
            for col in 1..cols - 1 {
                if data[index(row, col)] == A {
                    let da0 = data[index(row - 1, col - 1)];
                    let da1 = data[index(row, col)];
                    let da2 = data[index(row + 1, col + 1)];
                    let db0 = data[index(row - 1, col + 1)];
                    let db1 = data[index(row, col)];
                    let db2 = data[index(row + 1, col - 1)];

                    let da = [da0, da1, da2];
                    let db = [db0, db1, db2];

                    if (da == MAS || da == SAM) && (db == MAS || db == SAM) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    use aoc_traits::AdventOfCodeDay;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_part1() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 18);
    }

    #[test]
    fn test_part2() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 9);
    }
}
