use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn next(&self) -> [Self; 3] {
        match self {
            Dir::North => [Dir::West, Dir::North, Dir::East],
            Dir::East => [Dir::North, Dir::East, Dir::South],
            Dir::South => [Dir::East, Dir::South, Dir::West],
            Dir::West => [Dir::South, Dir::West, Dir::North],
        }
    }

    pub fn offset(&self, row: usize, col: usize) -> (usize, usize) {
        match self {
            Dir::West => (row, col - 1),
            Dir::East => (row, col + 1),
            Dir::North => (row - 1, col),
            Dir::South => (row + 1, col),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Position,
    direction: Dir,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Position = (usize, usize);

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = (Vec<Vec<u8>>, Position, Position);
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let mut start = None;
        let mut end = None;
        let map = input
            .lines()
            .enumerate()
            .map(|(r, row)| {
                row.as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(c, e)| {
                        if *e == b'S' {
                            start = Some((r, c));
                            b'.'
                        } else if *e == b'E' {
                            end = Some((r, c));
                            b'.'
                        } else {
                            *e
                        }
                    })
                    .collect()
            })
            .collect();
        (map, start.unwrap(), end.unwrap())
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        todo!()
        // let map = &input.0;
        // let start = input.1;
        // let end = input.2;

        // let rows = map.len();
        // let cols = map[0].len();

        // let mut dist = HashMap::new();
        // for row in 0..rows {
        //     for col in 0..cols {
        //         dist.insert((row, col), usize::MAX - 1);
        //     }
        // }

        // let mut heap = BinaryHeap::new();

        // // We're at `start`, with a zero cost
        // // dist.insert(start, 0);
        // heap.push(State {
        //     cost: 0,
        //     position: start,
        //     direction: Dir::East,
        // });

        // // Examine the frontier with lower cost nodes first (min-heap)
        // while let Some(State {
        //     cost,
        //     position,
        //     direction,
        // }) = heap.pop()
        // {
        //     // Alternatively we could have continued to find all shortest paths
        //     println!("cur pos {position:?}");
        //     if position == end {
        //         return cost;
        //     }

        //     // Important as we may have already found a better way
        //     if cost > *dist.get(&position).unwrap() {
        //         continue;
        //     }

        //     // For each node we can reach, see if we can find a way with
        //     // a lower cost going through this node
        //     for dir in direction.next() {
        //         println!("checking {dir:?}");
        //         let (row, col, cost_) = if dir == direction {
        //             println!("same dir -> advance");
        //             let (row, col) = dir.offset(position.0, position.1);
        //             if map[row][col] == b'#' {
        //                 println!("wall, so cont");
        //                 (row, col, usize::MAX)
        //             } else {
        //                 (row, col, 1)
        //             }
        //         } else {
        //             println!("left or eight dir -> just turn");
        //             (position.0, position.1, 1000)
        //         };

        //         let next = State {
        //             cost: cost.saturating_add(cost_),
        //             position: (row, col),
        //             direction: dir,
        //         };

        //         // If so, add it to the frontier and continue
        //         if next.cost < *dist.get(&next.position).unwrap() {
        //             println!("pushed {row}, {col} with cost {cost_} dir = {dir:?}");
        //             heap.push(next);
        //             // Relaxation, we have now found a better way
        //             dist.insert(next.position, next.cost);
        //         }
        //     }
        // }
        // 0
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    use aoc_traits::AdventOfCodeDay;

    const INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    #[test]
    fn test_part1() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 11);
    }

    #[test]
    fn test_part2() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 0);
    }
}
