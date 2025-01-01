#[allow(unused_imports)]
use advent_of_code::*;
use nalgebra::{Matrix2, Vector2};

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve_machines(input, 0))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve_machines(input, 10000000000000))
}

type Machine = ((usize, usize), (usize, usize), (usize, usize));

fn solve_machines(input: &str, pmod: usize) -> usize {
    parser!(section(
        line("Button A: X+" usize ", Y+" usize)
        line("Button B: X+" usize ", Y+" usize)
        line("Prize: X=" usize ", Y=" usize)
    )*)
    .parse(input)
    .unwrap()
    .into_iter()
    .map(|machine| solve(machine, pmod))
    .sum()
}

fn solve(machine: Machine, pmod: usize) -> usize {
    let (a, b, mut p) = machine;
    p.0 += pmod;
    p.1 += pmod;

    let m = Matrix2::new(a.0 as f64, b.0 as f64, a.1 as f64, b.1 as f64)
        .try_inverse()
        .unwrap()
        * Vector2::new(p.0 as f64, p.1 as f64);

    let at = (m.x.round() as usize);
    let bt = (m.y.round() as usize);
    if at * a.0 + bt * b.0 == p.0 && at * a.1 + bt * b.1 == p.1 {
        3 * at + bt
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
