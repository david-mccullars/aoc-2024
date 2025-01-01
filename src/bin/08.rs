#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::{HashMap, HashSet};
use itertools::*;
use std::cmp;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    Some(Antenna::new(input).antinodes(false).len())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Antenna::new(input).antinodes(true).len())
}

struct Antenna {
    locations: Grid,
}

impl Antenna {
    fn new(input: &str) -> Self {
        let locations = parser!(std_grid(alnum)).parse(input).unwrap();
        Self { locations }
    }

    fn antinodes(&self, use_resonant_harmonics: bool) -> HashSet<Pos> {
        let mut set = HashSet::new();
        for (c, positions) in &self.locations.map {
            for combo in positions.iter().combinations(2) {
                let dx = combo[0].0 - combo[1].0;
                let dy = combo[0].1 - combo[1].1;

                if use_resonant_harmonics {
                    set.insert(*combo[0]);
                    set.insert(*combo[1]);
                }

                for i in (1..) {
                    let a = (combo[1].0 - dx * i, combo[1].1 - dy * i);
                    if self.is_on_map(&a) {
                        set.insert(a);
                    } else {
                        break;
                    }
                    if !use_resonant_harmonics {
                        break;
                    }
                }

                for i in (1..) {
                    let a = (combo[0].0 + dx * i, combo[0].1 + dy * i);
                    if self.is_on_map(&a) {
                        set.insert(a);
                    } else {
                        break;
                    }
                    if !use_resonant_harmonics {
                        break;
                    }
                }
            }
        }

        set
    }

    fn is_on_map(&self, pos: &Pos) -> bool {
        pos.0 >= 0
            && pos.1 >= 0
            && pos.0 <= self.locations.bounds.0
            && pos.1 <= self.locations.bounds.1
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
        assert_eq!(result, Some(34));
    }
}
