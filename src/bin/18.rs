#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use pathfinding::directed::bfs::bfs;
use rayon::prelude::*;

advent_of_code::solution!(18);

#[cfg(test)]
const GRID_MAX: isize = 6;
#[cfg(not(test))]
const GRID_MAX: isize = 70;

#[cfg(test)]
const START_TIME: usize = 12;
#[cfg(not(test))]
const START_TIME: usize = 1024;

pub fn part_one(input: &str) -> Option<usize> {
    Memory::new(input).min_path()
}

pub fn part_two(input: &str) -> Option<String> {
    let mut memory = Memory::new(input);
    (1..=memory.time_left())
        .into_par_iter()
        .find_first(|t| memory.clone().advance_time(*t).min_path().is_none())
        .map(|time| {
            let (x, y) = memory.automaton[memory.time + time - 1];
            format!("{},{}", x, y).into()
        })
}

#[derive(Debug, Clone)]
struct Memory {
    time: usize,
    corrupted: HashSet<Pos>,
    automaton: Vec<Pos>,
}

impl Memory {
    fn new(input: &str) -> Self {
        let automaton = parser!(lines(isize "," isize)).parse(input).unwrap();
        Self {
            time: 0,
            corrupted: HashSet::new(),
            automaton,
        }
        .advance_time(START_TIME)
    }

    fn advance_time(mut self, amount: usize) -> Self {
        for t in (0..amount) {
            if self.time + t < self.automaton.len() {
                self.corrupted.insert(self.automaton[self.time + t]);
            }
        }
        self.time += amount;
        self
    }

    fn time_left(&self) -> usize {
        self.automaton.len() - self.time
    }

    fn min_path(&self) -> Option<usize> {
        let start = (0, 0);
        let mut successors = move |&pos: &Pos| {
            DIRECTIONS.iter().filter_map(move |dir| {
                let pos2 = dir.forward_from(&pos);
                (is_on_map(&pos2) && !self.corrupted.contains(&pos2)).then_some(pos2)
            })
        };
        let mut success = |&pos: &Pos| pos.0 == GRID_MAX && pos.1 == GRID_MAX;

        bfs(&start, successors, success).map(|path| path.len() - 1)
    }
}

fn is_on_map(pos: &Pos) -> bool {
    (0..=GRID_MAX).contains(&pos.0) && (0..=GRID_MAX).contains(&pos.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_owned()));
    }
}
