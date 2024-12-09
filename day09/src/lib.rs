use aoc_traits::AdventOfCodeDay;

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
        let mut disk_map = vec![];
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

        for i in (0..disk_map.len()).rev() {
            let d = disk_map[i];
            if d.is_some() {
                for j in 0..i {
                    if disk_map[j].is_none() {
                        disk_map[j] = d;
                        disk_map[i] = None;
                        break;
                    }
                }
                if disk_map[i].is_some() {
                    break;
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
        todo!()
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
        assert_eq!(Solver::solve_part2(&parsed), 31);
    }
}
