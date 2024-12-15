use std::fmt::Display;

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Wall,
    Box,
    BoxL,
    BoxR,
    Empty,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Wall => write!(f, "#"),
            Cell::Box => write!(f, "O"),
            Cell::Empty => write!(f, "."),
            Cell::BoxL => write!(f, "["),
            Cell::BoxR => write!(f, "]"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = (Vec<Vec<Cell>>, (usize, usize), Vec<Move>);
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let (map, moves) = input.split_once("\n\n").unwrap();
        let mut pos = None;
        let map = map
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| match c {
                        '#' => Cell::Wall,
                        'O' => Cell::Box,
                        '.' => Cell::Empty,
                        '@' => {
                            pos = Some((row, col));
                            Cell::Empty
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        let moves = moves
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '>' => Move::Right,
                    '<' => Move::Left,
                    'v' => Move::Down,
                    '^' => Move::Up,
                    _ => unreachable!(),
                })
            })
            .collect();
        (map, pos.unwrap(), moves)
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let mut map = input.0.clone();
        let (mut row, mut col) = input.1;
        let moves = &input.2;

        for m in moves {
            match m {
                Move::Left => match map[row][col - 1] {
                    Cell::Wall => {}
                    Cell::Box => {
                        if let Some(pos) = map[row]
                            .iter()
                            .take(col)
                            .rev()
                            .take_while(|c| matches!(c, Cell::Empty | Cell::Box))
                            .position(|c| matches!(c, Cell::Empty))
                        {
                            map[row][col - pos - 1] = Cell::Box;
                            map[row][col - 1] = Cell::Empty;
                            col -= 1;
                        }
                    }
                    Cell::Empty => {
                        col -= 1;
                    }
                    _ => unreachable!(),
                },
                Move::Right => match map[row][col + 1] {
                    Cell::Wall => {}
                    Cell::Box => {
                        if let Some(pos) = map[row]
                            .iter()
                            .skip(col + 1)
                            .take_while(|c| matches!(c, Cell::Empty | Cell::Box))
                            .position(|c| matches!(c, Cell::Empty))
                        {
                            map[row][col + pos + 1] = Cell::Box;
                            map[row][col + 1] = Cell::Empty;
                            col += 1;
                        }
                    }
                    Cell::Empty => {
                        col += 1;
                    }
                    _ => unreachable!(),
                },
                Move::Up => match map[row - 1][col] {
                    Cell::Wall => {}
                    Cell::Box => {
                        if let Some(pos) = (0..row)
                            .map(|r| map[r][col])
                            .rev()
                            .take_while(|c| matches!(c, Cell::Empty | Cell::Box))
                            .position(|c| matches!(c, Cell::Empty))
                        {
                            map[row - pos - 1][col] = Cell::Box;
                            map[row - 1][col] = Cell::Empty;
                            row -= 1;
                        }
                    }
                    Cell::Empty => {
                        row -= 1;
                    }
                    _ => unreachable!(),
                },
                Move::Down => match map[row + 1][col] {
                    Cell::Wall => {}
                    Cell::Box => {
                        if let Some(pos) = (row + 1..map.len())
                            .map(|r| map[r][col])
                            .take_while(|c| matches!(c, Cell::Empty | Cell::Box))
                            .position(|c| matches!(c, Cell::Empty))
                        {
                            map[row + pos + 1][col] = Cell::Box;
                            map[row + 1][col] = Cell::Empty;
                            row += 1;
                        }
                    }
                    Cell::Empty => {
                        row += 1;
                    }
                    _ => unreachable!(),
                },
            }
        }

        let mut sum = 0;
        for (r, row_) in map.iter().enumerate() {
            for (c, cell) in row_.iter().enumerate() {
                if let Cell::Box = cell {
                    sum += 100 * r + c;
                }
            }
        }
        sum
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let (mut row, mut col) = input.1;
        let moves = &input.2;

        let mut map = Vec::with_capacity(input.0.len());
        for row in &input.0 {
            let mut row_ = Vec::with_capacity(row.len() * 2);
            for c in row {
                match c {
                    Cell::Wall => {
                        row_.push(Cell::Wall);
                        row_.push(Cell::Wall);
                    }
                    Cell::Box => {
                        row_.push(Cell::BoxL);
                        row_.push(Cell::BoxR);
                    }
                    Cell::Empty => {
                        row_.push(Cell::Empty);
                        row_.push(Cell::Empty);
                    }
                    _ => unreachable!(),
                }
            }
            map.push(row_);
        }

        col *= 2;

        // for (r, row_) in map.iter().enumerate() {
        //     for (c, cell) in row_.iter().enumerate() {
        //         if r == row && c == col {
        //             print!("@");
        //         } else {
        //             print!("{cell}");
        //         }
        //     }
        //     println!();
        // }

        for m in moves {
            // println!("move = {m:?}");
            match m {
                Move::Left => match map[row][col - 1] {
                    Cell::Wall => {}
                    Cell::BoxR => {
                        if let Some(pos) = map[row]
                            .iter()
                            .take(col)
                            .rev()
                            .take_while(|c| matches!(c, Cell::Empty | Cell::BoxL | Cell::BoxR))
                            .position(|c| matches!(c, Cell::Empty))
                        {
                            for i in (col - pos - 1..col - 1).step_by(2) {
                                map[row][i] = Cell::BoxL;
                                map[row][i + 1] = Cell::BoxR;
                            }
                            map[row][col - 1] = Cell::Empty;
                            col -= 1;
                        }
                    }
                    Cell::Empty => {
                        col -= 1;
                    }
                    _ => unreachable!(),
                },
                Move::Right => match map[row][col + 1] {
                    Cell::Wall => {}
                    Cell::BoxL => {
                        if let Some(pos) = map[row]
                            .iter()
                            .skip(col + 1)
                            .take_while(|c| matches!(c, Cell::Empty | Cell::BoxL | Cell::BoxR))
                            .position(|c| matches!(c, Cell::Empty))
                        {
                            for i in (col + 2..col + pos + 1).step_by(2) {
                                map[row][i] = Cell::BoxL;
                                map[row][i + 1] = Cell::BoxR;
                            }
                            map[row][col + 1] = Cell::Empty;
                            col += 1;
                        }
                    }
                    Cell::Empty => {
                        col += 1;
                    }
                    _ => unreachable!(),
                },
                Move::Up => match map[row - 1][col] {
                    Cell::Wall => {}
                    Cell::BoxL => {
                        if let Some(pos) = (0..row)
                            .map(|r| map[r][col])
                            .rev()
                            .take_while(|c| matches!(c, Cell::Empty | Cell::BoxL | Cell::BoxR))
                            .position(|c| matches!(c, Cell::Empty))
                        {
                            todo!()
                        }
                    }
                    Cell::BoxR => {
                        if let Some(pos) = (0..row)
                            .map(|r| map[r][col])
                            .rev()
                            .take_while(|c| matches!(c, Cell::Empty | Cell::BoxL | Cell::BoxR))
                            .position(|c| matches!(c, Cell::Empty))
                        {
                            todo!()
                        }
                    }
                    Cell::Empty => {
                        row -= 1;
                    }
                    _ => unreachable!(),
                },
                Move::Down => match map[row + 1][col] {
                    Cell::Wall => {}
                    Cell::BoxL => {
                        if let Some(pos) = (row + 1..map.len())
                            .map(|r| map[r][col])
                            .take_while(|c| matches!(c, Cell::Empty | Cell::Box))
                            .position(|c| matches!(c, Cell::Empty))
                        {
                            todo!()
                        }
                    }
                    Cell::BoxR => {
                        if let Some(pos) = (row + 1..map.len())
                            .map(|r| map[r][col])
                            .take_while(|c| matches!(c, Cell::Empty | Cell::Box))
                            .position(|c| matches!(c, Cell::Empty))
                        {
                            todo!()
                        }
                    }
                    Cell::Empty => {
                        row += 1;
                    }
                    _ => unreachable!(),
                },
            }
            // for (r, row_) in map.iter().enumerate() {
            //     for (c, cell) in row_.iter().enumerate() {
            //         if r == row && c == col {
            //             print!("@");
            //         } else {
            //             print!("{cell}");
            //         }
            //     }
            //     println!();
            // }
        }

        let mut sum = 0;
        for (r, row_) in map.iter().enumerate() {
            for (c, cell) in row_.iter().enumerate() {
                if let Cell::Box = cell {
                    sum += 100 * r + c;
                }
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    use aoc_traits::AdventOfCodeDay;

    const INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    fn test_part1() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 10092);
    }

    #[test]
    fn test_part2() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 9021);
    }
}
