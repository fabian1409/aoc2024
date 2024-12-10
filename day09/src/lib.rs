use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, Copy)]
enum Space {
    Used { id: u32, len: u8 },
    Free { len: u8 },
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<u8>;
    type Part1Output = u64;
    type Part2Output = u64;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input.as_bytes().iter().map(|d| d - 48).collect()
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let mut disk_map = Vec::with_capacity(input.len());
        let mut id = 0;
        for (i, d) in input.iter().enumerate() {
            if i % 2 == 0 {
                for _ in 0..*d as usize {
                    disk_map.push(Some(id));
                }
                id += 1;
            } else {
                for _ in 0..*d as usize {
                    disk_map.push(None);
                }
            }
        }
        let mut start = 0;
        for i in (0..disk_map.len()).rev() {
            let d = disk_map[i];
            if d.is_some() {
                for j in start..i {
                    if disk_map[j].is_none() {
                        disk_map[j] = d;
                        disk_map[i] = None;
                        start = j + 1;
                        break;
                    }
                }
            }
        }

        disk_map
            .into_iter()
            .enumerate()
            .map(|(i, d)| i as u64 * d.unwrap_or(0) as u64)
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let mut disk_map = Vec::with_capacity(input.len());
        let mut id = 0;
        for (i, d) in input.iter().enumerate() {
            if i % 2 == 0 {
                disk_map.push(Space::Used { id, len: *d });
                id += 1;
            } else {
                disk_map.push(Space::Free { len: *d });
            }
        }

        for i in (0..disk_map.len()).rev() {
            let d = disk_map[i];
            if let Space::Used { id: _, len } = d {
                for j in 0..i {
                    if let Space::Free { len: free_len } = disk_map[j] {
                        if free_len >= len {
                            disk_map[j] = d;
                            disk_map[i] = Space::Free { len };
                            if free_len > len {
                                disk_map.insert(
                                    j + 1,
                                    Space::Free {
                                        len: free_len - len,
                                    },
                                );
                            }
                            break;
                        }
                    }
                }
            }
        }

        let mut i = 0;
        disk_map
            .into_iter()
            .map(|d| {
                let mut sum = 0;
                match d {
                    Space::Used { id, len } => {
                        for _ in 0..len as usize {
                            sum += id as u64 * i;
                            i += 1;
                        }
                    }
                    Space::Free { len } => i += len as u64,
                }
                sum
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    use aoc_traits::AdventOfCodeDay;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 1928);
    }

    #[test]
    fn test_part2() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 2858);
    }
}
