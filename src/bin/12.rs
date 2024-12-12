#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        Farm::new(input)
            .regions()
            .into_iter()
            .map(|region| region.area() * region.perimeter())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        Farm::new(input)
            .regions()
            .into_iter()
            .map(|region| region.area() * region.sides())
            .sum(),
    )
}

struct Farm {
    plots: HashMap<Pos, char>,
}

impl Farm {
    fn new(input: &str) -> Self {
        let mut plots = HashMap::new();
        for (y, row) in input.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                plots.insert(pos_from(x, y), c);
            }
        }
        Self { plots }
    }

    fn regions(&self) -> Vec<Region> {
        let mut regions = vec![];
        let mut viewed = HashSet::new();
        for (pos, c) in &self.plots {
            if !viewed.contains(pos) {
                regions.push(self.determine_region(&pos, *c, &mut viewed));
            }
        }
        regions
    }

    fn determine_region(&self, start: &Pos, c: char, viewed: &mut HashSet<Pos>) -> Region {
        let mut region = Region::default();

        viewed.insert(*start);
        flood_fill(
            start,
            |_| region.area += 1,
            |pos, pos2, dir| {
                if self.plots.get(pos2) == Some(&c) {
                    viewed.insert(*pos2)
                } else {
                    region.border.push((*dir, *pos2));
                    false
                }
            },
        );

        region
    }
}

#[derive(Default)]
struct Region {
    area: usize,
    border: Vec<(Direction, Pos)>,
}

impl Region {
    fn area(&self) -> usize {
        self.area
    }

    fn perimeter(&self) -> usize {
        self.border.len()
    }

    fn sides(&self) -> usize {
        self.border
            .clone()
            .into_iter()
            .into_group_map()
            .into_iter()
            .map(|(dir, mut plots)| {
                match dir {
                    Direction::North | Direction::South => {
                        plots = plots.into_iter().map(|(x, y)| (y, x)).collect();
                    }
                    Direction::East | Direction::West => {}
                }

                plots
                    .into_iter()
                    .into_group_map()
                    .into_iter()
                    .map(|(_, mut nums)| {
                        nums.sort();
                        1 + nums.windows(2).filter(|a| a[0] + 1 != a[1]).count()
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_one_c() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(436));
    }

    #[test]
    fn test_part_two_c() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1206));
    }
}
