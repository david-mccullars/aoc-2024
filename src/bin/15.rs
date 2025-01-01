#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::HashSet;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<isize> {
    Some(Warehouse::new(input, false).run().boxes_gps())
}

pub fn part_two(input: &str) -> Option<isize> {
    Some(Warehouse::new(input, true).run().boxes_gps())
}

fn widen(input: &str) -> String {
    input.chars().fold("".to_owned(), |mut s, c| {
        s.push(c);
        match c {
            '#' => {
                s.push(c);
            }
            'O' | '@' | '.' => {
                s.push('.');
            }
            _ => {}
        }
        s
    })
}

#[derive(Debug)]
struct Warehouse {
    walls: HashSet<Pos>,
    boxes: HashSet<Pos>,
    robot: Pos,
    movements: Vec<Direction>,
    wide: bool,
}

impl Warehouse {
    fn new(input: &str, wide: bool) -> Self {
        let input = if wide { &widen(input) } else { input };

        let (mut grid, movements) = parser!(
            section(grid_of(".#O@"))
            section(lines((c:any_char => Direction::from_char(c))+))
        )
        .parse(input)
        .unwrap();

        Self {
            walls: grid.take_all('#'),
            boxes: grid.take_all('O'),
            robot: grid.take_one('@'),
            movements: movements.into_iter().flatten().collect(),
            wide,
        }
    }

    fn run(mut self) -> Self {
        for dir in self.movements.clone() {
            let pos2 = dir.forward_from(&self.robot);
            let pos2w = west(&pos2);

            if self.walls.contains(&pos2) {
                continue;
            }

            if self.boxes.contains(&pos2) {
                let mut backup = self.boxes.clone();
                if !self.attempt_push(&pos2, &dir) {
                    self.boxes = backup;
                    continue;
                }
            } else if self.wide && self.boxes.contains(&pos2w) {
                let mut backup = self.boxes.clone();
                if !self.attempt_push(&pos2w, &dir) {
                    self.boxes = backup;
                    continue;
                }
            }

            self.robot = pos2;
        }
        self
    }

    fn attempt_push(&mut self, pos: &Pos, dir: &Direction) -> bool {
        let pos2 = dir.forward_from(&pos);
        let pos2e = east(&pos2);
        let pos2w = west(&pos2);

        if self.walls.contains(&pos2) {
            return false;
        } else if self.wide && self.walls.contains(&pos2e) {
            return false;
        }

        if !self.wide {
            if self.boxes.contains(&pos2) {
                if !self.attempt_push(&pos2, dir) {
                    return false;
                }
            }
        } else {
            match dir {
                Direction::North | Direction::South => {
                    if self.boxes.contains(&pos2) {
                        if !self.attempt_push(&pos2, dir) {
                            return false;
                        }
                    }
                    if self.boxes.contains(&pos2w) {
                        if !self.attempt_push(&pos2w, dir) {
                            return false;
                        }
                    }
                    if self.boxes.contains(&pos2e) {
                        if !self.attempt_push(&pos2e, dir) {
                            return false;
                        }
                    }
                }
                Direction::East => {
                    if self.boxes.contains(&pos2e) {
                        if !self.attempt_push(&pos2e, dir) {
                            return false;
                        }
                    }
                }
                Direction::West => {
                    if self.boxes.contains(&pos2w) {
                        if !self.attempt_push(&pos2w, dir) {
                            return false;
                        }
                    }
                }
            }
        }

        self.boxes.remove(pos);
        self.boxes.insert(pos2);
        true
    }

    fn boxes_gps(&self) -> isize {
        self.boxes.iter().map(|pos| 100 * pos.1 + pos.0).sum()
    }
}

#[inline]
fn east(pos: &Pos) -> Pos {
    Direction::East.forward_from(pos)
}

#[inline]
fn west(pos: &Pos) -> Pos {
    Direction::West.forward_from(pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sm() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_lg() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9021));
    }
}
