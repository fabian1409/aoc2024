use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = (Vec<Vec<u8>>, (usize, usize), Direction);
    type Part1Output = u32;
    type Part2Output = u32;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let mut start = None;
        let mut dir = None;
        (
            input
                .lines()
                .enumerate()
                .map(|(row, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(col, c)| {
                            if c == '^' {
                                start = Some((row, col));
                                dir = Some(Direction::Up);
                            } else if c == 'v' {
                                start = Some((row, col));
                                dir = Some(Direction::Down);
                            } else if c == '<' {
                                start = Some((row, col));
                                dir = Some(Direction::Left);
                            } else if c == '>' {
                                start = Some((row, col));
                                dir = Some(Direction::Right);
                            }
                            c as u8
                        })
                        .collect()
                })
                .collect(),
            start.unwrap(),
            dir.unwrap(),
        )
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let data = &input.0;
        let rows = data.len();
        let cols = data[0].len();
        let (mut r, mut c) = input.1;
        let mut dir = input.2;

        let mut visited = vec![vec![false; cols]; rows];
        visited[r][c] = true;

        loop {
            let offset = dir.offset();
            let next_r = r as isize + offset.0;
            let next_c = c as isize + offset.1;

            if next_r >= rows as isize || next_r < 0 || next_c >= cols as isize || next_c < 0 {
                break;
            }

            let peek = data[next_r as usize][next_c as usize];

            if peek == b'#' {
                dir = match dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                }
            } else {
                r = next_r as usize;
                c = next_c as usize;
                visited[r][c] = true;
            }
        }

        visited
            .iter()
            .flatten()
            .map(|v| if *v { 1 } else { 0 })
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    use aoc_traits::AdventOfCodeDay;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_part1() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 41);
    }

    #[test]
    fn test_part2() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 31);
    }
}
