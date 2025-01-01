#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(14);

const TEST_DIMS: Pos = (11, 7);
const FULL_DIMS: Pos = (101, 103);

pub fn part_one(input: &str) -> Option<usize> {
    _part_one(input, FULL_DIMS)
}

pub fn _part_one(input: &str, dims: Pos) -> Option<usize> {
    let robots = Robots::new(input, dims);
    Some(robots.safety_score(100))
}

pub fn part_two(input: &str) -> Option<isize> {
    let dims = FULL_DIMS;
    let robots = Robots::new(input, dims);
    let mut min_s = usize::MAX;
    let mut min_n = None;
    for n in (0..(dims.0 * dims.1)) {
        let s = robots.safety_score(n);
        if s < min_s {
            min_s = s;
            min_n = Some(n);
        }
    }
    min_n
}

type Robot = (Pos, Pos);

#[derive(Debug)]
struct Robots {
    robots: Vec<Robot>,
    dims: Pos,
    mid: Pos,
}

impl Robots {
    fn new(input: &str, dims: Pos) -> Self {
        let robots = parser!(lines(
            "p=" px:isize "," py:isize
            " v=" vx:isize "," vy:isize
            => ((px, py), (vx, vy)),
        ))
        .parse(input)
        .unwrap();
        let mid = (dims.0 / 2, dims.1 / 2);
        Self { robots, dims, mid }
    }

    fn safety_score(&self, n: isize) -> usize {
        let mut quads = [0; 4];
        for (p, v) in &self.robots {
            let mut x = (p.0 + v.0 * n) % self.dims.0;
            while x < 0 {
                x += self.dims.0;
            }
            let mut y = (p.1 + v.1 * n) % self.dims.1;
            while y < 0 {
                y += self.dims.1;
            }
            if let Some(q) = self.quadrant((x, y)) {
                quads[q] += 1;
            }
        }
        quads[0] * quads[1] * quads[2] * quads[3]
    }

    fn quadrant(&self, pos: Pos) -> Option<usize> {
        if pos.0 == self.mid.0 || pos.1 == self.mid.1 {
            None
        } else if pos.0 < self.mid.0 {
            if pos.1 < self.mid.1 {
                Some(0)
            } else {
                Some(1)
            }
        } else {
            if pos.1 < self.mid.1 {
                Some(2)
            } else {
                Some(3)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = _part_one(
            &advent_of_code::template::read_file("examples", DAY),
            TEST_DIMS,
        );
        assert_eq!(result, Some(12));
    }
}
