#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::HashSet;
use rayon::prelude::*;
use std::collections::VecDeque;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    Some(TopoMap::new(input).count_all_hikes())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(TopoMap::new(input).count_all_distinct_hikes())
}

struct TopoMap {
    map: Vec<Vec<u8>>,
    starts: Vec<Pos>,
}

impl TopoMap {
    fn new(input: &str) -> Self {
        let map = input
            .lines()
            .map(|row| row.as_bytes().iter().map(|b| *b - b'0').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut starts = vec![];

        for (y, row) in map.iter().enumerate() {
            for (x, d) in row.iter().enumerate() {
                if *d == 0 {
                    starts.push(pos_from(x, y));
                }
            }
        }

        Self { map, starts }
    }

    fn count_all_hikes(&self) -> usize {
        self.starts
            .par_iter()
            .map(|start| {
                self.reachable_from(start)
                    .iter()
                    .collect::<HashSet<_>>()
                    .len()
            })
            .sum()
    }

    fn count_all_distinct_hikes(&self) -> usize {
        self.starts
            .par_iter()
            .map(|start| self.reachable_from(start).len())
            .sum()
    }

    fn reachable_from(&self, start: &Pos) -> Vec<Pos> {
        let mut seen = Vec::new();
        let mut queue = VecDeque::from([*start]);

        while let Some(pos) = queue.pop_front() {
            if let Some(h) = self.at(&pos) {
                if h == 9 {
                    seen.push(pos);
                    continue;
                }
                for dir in DIRECTIONS {
                    let pos2 = dir.forward_from(&pos);
                    if let Some(h2) = self.at(&pos2) {
                        if h2 == h + 1 {
                            queue.push_back(pos2);
                        }
                    }
                }
            }
        }

        seen
    }

    #[inline]
    fn at(&self, pos: &Pos) -> Option<u8> {
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        }

        let x: usize = pos.0 as usize;
        let y: usize = pos.1 as usize;
        if y >= self.map.len() || x >= self.map[y].len() {
            None
        } else {
            Some(self.map[y][x])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
