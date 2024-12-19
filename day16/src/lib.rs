use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Position,
    direction: Dir,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Position = (usize, usize);

fn walk(
    predecessors: &HashMap<(Position, Dir), Vec<(Position, Dir)>>,
    pos: (Position, Dir),
    path: &mut HashSet<(Position, Dir)>,
) {
    let preds = predecessors.get(&pos).unwrap();
    path.insert(pos);
    for pred in preds {
        if !path.contains(pred) {
            walk(predecessors, *pred, path);
        }
    }
}

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
        let map = &input.0;
        let start = input.1;
        let end = input.2;

        let rows = map.len();
        let cols = map[0].len();

        let mut dist = HashMap::new();
        for row in 0..rows {
            for col in 0..cols {
                for dir in [Dir::North, Dir::East, Dir::South, Dir::West] {
                    dist.insert(((row, col), dir), usize::MAX);
                }
            }
        }

        dist.insert((start, Dir::East), 0);

        let mut heap = BinaryHeap::new();
        heap.push(State {
            cost: 0,
            position: start,
            direction: Dir::East,
        });

        while let Some(State {
            cost,
            position,
            direction,
        }) = heap.pop()
        {
            if position == end {
                return cost;
            }

            if cost > *dist.get(&(position, direction)).unwrap() {
                continue;
            }

            for dir in direction.next() {
                let (row, col, cost_) = if dir == direction {
                    let (row, col) = dir.offset(position.0, position.1);
                    if map[row][col] != b'#' {
                        (row, col, 1)
                    } else {
                        continue;
                    }
                } else {
                    (position.0, position.1, 1000)
                };

                let next = State {
                    cost: cost.saturating_add(cost_),
                    position: (row, col),
                    direction: dir,
                };

                let cur_cost = *dist.get(&(next.position, dir)).unwrap();
                if next.cost < cur_cost {
                    heap.push(next);
                    dist.insert((next.position, dir), next.cost);
                }
            }
        }
        0
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let map = &input.0;
        let start = input.1;
        let end = input.2;

        let rows = map.len();
        let cols = map[0].len();

        let mut dist = HashMap::new();
        let mut predecessors = HashMap::new();
        for row in 0..rows {
            for col in 0..cols {
                for dir in [Dir::North, Dir::East, Dir::South, Dir::West] {
                    dist.insert(((row, col), dir), usize::MAX);
                    predecessors.insert(((row, col), dir), Vec::new());
                }
            }
        }

        dist.insert((start, Dir::East), 0);

        let mut heap = BinaryHeap::new();
        heap.push(State {
            cost: 0,
            position: start,
            direction: Dir::East,
        });

        while let Some(State {
            cost,
            position,
            direction,
        }) = heap.pop()
        {
            if cost > *dist.get(&(position, direction)).unwrap() {
                continue;
            }

            for dir in direction.next() {
                let (row, col, cost_) = if dir == direction {
                    let (row, col) = dir.offset(position.0, position.1);
                    if map[row][col] != b'#' {
                        (row, col, 1)
                    } else {
                        continue;
                    }
                } else {
                    (position.0, position.1, 1000)
                };

                let next = State {
                    cost: cost.saturating_add(cost_),
                    position: (row, col),
                    direction: dir,
                };

                let cur_cost = *dist.get(&(next.position, dir)).unwrap();
                if next.cost < cur_cost {
                    heap.push(next);
                    dist.insert((next.position, dir), next.cost);
                    predecessors.insert((next.position, dir), vec![(position, direction)]);
                } else if next.cost == cur_cost {
                    predecessors
                        .entry((next.position, dir))
                        .or_default()
                        .push((position, direction));
                }
            }
        }

        let mut path = HashSet::new();
        for dir in [Dir::North, Dir::East, Dir::South, Dir::West] {
            walk(&predecessors, (end, dir), &mut path);
        }

        let mut path_ = HashSet::new();
        for (pos, _) in path.iter() {
            path_.insert(pos);
        }

        path_.len()
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
        assert_eq!(Solver::solve_part1(&parsed), 7036);
    }

    #[test]
    fn test_part2() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 45);
    }
}
