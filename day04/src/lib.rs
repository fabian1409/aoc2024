use aoc_traits::AdventOfCodeDay;

const X: u8 = 88;
const M: u8 = 77;
const A: u8 = 65;
const S: u8 = 83;

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
        let mut cols = 0;
        let data = input
            .lines()
            .flat_map(|line| {
                let row = line.as_bytes().to_vec();
                cols = row.len();
                rows += 1;
                row
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
                if data[index(row, col)] == X {
                    let north = row >= 3;
                    let east = col < cols - 3;
                    let south = row < rows - 3;
                    let west = col >= 3;
                    if north {
                        let n0 = data[index(row - 1, col)];
                        let n1 = data[index(row - 2, col)];
                        let n2 = data[index(row - 3, col)];
                        if [n0, n1, n2] == MAS {
                            count += 1;
                        }
                    }

                    if north && east {
                        let ne0 = data[index(row - 1, col + 1)];
                        let ne1 = data[index(row - 2, col + 2)];
                        let ne2 = data[index(row - 3, col + 3)];
                        if [ne0, ne1, ne2] == MAS {
                            count += 1;
                        }
                    }

                    if east {
                        let e0 = data[index(row, col + 1)];
                        let e1 = data[index(row, col + 2)];
                        let e2 = data[index(row, col + 3)];
                        if [e0, e1, e2] == MAS {
                            count += 1;
                        }
                    }

                    if south && east {
                        let se0 = data[index(row + 1, col + 1)];
                        let se1 = data[index(row + 2, col + 2)];
                        let se2 = data[index(row + 3, col + 3)];
                        if [se0, se1, se2] == MAS {
                            count += 1;
                        }
                    }

                    if south {
                        let s0 = data[index(row + 1, col)];
                        let s1 = data[index(row + 2, col)];
                        let s2 = data[index(row + 3, col)];
                        if [s0, s1, s2] == MAS {
                            count += 1;
                        }
                    }

                    if south && west {
                        let sw0 = data[index(row + 1, col - 1)];
                        let sw1 = data[index(row + 2, col - 2)];
                        let sw2 = data[index(row + 3, col - 3)];
                        if [sw0, sw1, sw2] == MAS {
                            count += 1;
                        }
                    }

                    if west {
                        let w0 = data[index(row, col - 1)];
                        let w1 = data[index(row, col - 2)];
                        let w2 = data[index(row, col - 3)];
                        if [w0, w1, w2] == MAS {
                            count += 1;
                        }
                    }

                    if north && west {
                        let nw0 = data[index(row - 1, col - 1)];
                        let nw1 = data[index(row - 2, col - 2)];
                        let nw2 = data[index(row - 3, col - 3)];
                        if [nw0, nw1, nw2] == MAS {
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
