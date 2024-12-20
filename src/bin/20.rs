#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::{HashMap, HashSet};
use pathfinding::directed::bfs::bfs;
use rayon::prelude::*;

advent_of_code::solution!(20);

#[cfg(test)]
const MIN_SAVE: isize = 8;
#[cfg(not(test))]
const MIN_SAVE: isize = 100;

pub fn part_one(input: &str) -> Option<usize> {
    Some(Racetrack::new(input).valid_cheats(2))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Racetrack::new(input).valid_cheats(20))
}

struct Racetrack {
    walls: HashSet<Pos>,
    start: Pos,
    end: Pos,
    bounds: Pos,
}

impl Racetrack {
    fn new(input: &str) -> Self {
        let mut grid = parser!(grid_of(".#SE")).parse(input).unwrap();
        Self {
            walls: grid.take_all('#'),
            start: grid.take_one('S'),
            end: grid.take_one('E'),
            bounds: grid.bounds,
        }
    }

    fn path(&self) -> HashMap<Pos, isize> {
        let start = self.start;
        let mut successors = move |&pos: &Pos| {
            DIRECTIONS.iter().filter_map(move |dir| {
                let pos2 = dir.forward_from(&pos);
                (!self.walls.contains(&pos2)).then_some(pos2)
            })
        };
        let mut success = |&pos: &Pos| pos == self.end;

        bfs(&start, successors, success)
            .expect("No shortest path found")
            .into_iter()
            .enumerate()
            .map(|(i, pos)| (pos, i as isize))
            .collect()
    }

    fn valid_cheats(&self, max_distance: isize) -> usize {
        let non_cheat = self.path();
        non_cheat
            .par_iter()
            .map(|(pos, i)| {
                self.valid_cheats_at(pos, max_distance, &|pos2, distance| {
                    i + MIN_SAVE + distance <= *non_cheat.get(pos2).unwrap_or(&0)
                })
            })
            .sum()
    }

    fn valid_cheats_at<F>(&self, pos: &Pos, max_distance: isize, filter: &F) -> usize
    where
        F: Fn(&Pos, isize) -> bool,
    {
        (2..=max_distance)
            .flat_map(|distance| {
                (-distance..distance).flat_map(move |dx| {
                    let dy = distance - dx.abs();
                    [(pos.0 - dx, pos.1 - dy), (pos.0 + dx, pos.1 + dy)]
                        .into_iter()
                        .filter(move |pos2| self.is_on_map(pos2) && filter(pos2, distance))
                })
            })
            .count()
    }

    fn is_on_map(&self, pos: &Pos) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 <= self.bounds.0 && pos.1 <= self.bounds.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2492));
    }
}
