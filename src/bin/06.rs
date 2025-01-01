#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::{HashMap, HashSet};
use rayon::prelude::*;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    Some(Map::new(input).first_visits().len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::new(input);
    Some(
        map.first_visits()
            .par_iter()
            .filter(|(pos, _)| **pos != map.pos)
            .filter(|(pos, dir)| {
                let mut map2 = map.clone();
                map2.obstacles.insert(**pos);
                map2.pos = dir.backward_from(*pos);
                map2.dir = **dir;
                map2.traverse()
            })
            .count(),
    )
}

#[derive(Debug, Clone)]
struct Map {
    obstacles: HashSet<Pos>,
    bounds: Pos,
    pos: Pos,
    dir: Direction,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut grid = parser!(grid_of(".#^")).parse(input).unwrap();

        Self {
            obstacles: grid.take_all('#'),
            bounds: grid.bounds,
            pos: grid.take_one('^'),
            dir: Direction::North,
        }
    }

    fn first_visits(&self) -> HashMap<Pos, Direction> {
        let mut visits = HashMap::new();
        self.clone()._traverse(&mut visits);
        visits
    }

    fn traverse(&mut self) -> bool {
        self._traverse(&mut HashMap::new())
    }

    fn _traverse(&mut self, first_visits: &mut HashMap<Pos, Direction>) -> bool {
        let mut north_turns = HashSet::new();
        loop {
            first_visits.try_insert(self.pos, self.dir);
            let in_front = self.dir.forward_from(&self.pos);
            if self.obstacles.contains(&in_front) {
                self.dir = self.dir.turn_right();
                if self.dir == Direction::North && !north_turns.insert(self.pos) {
                    return true;
                }
            } else if self.is_on_map(&in_front) {
                self.pos = in_front;
            } else {
                return false;
            }
        }
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
