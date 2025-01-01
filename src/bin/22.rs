#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::HashMap;
use itertools::Itertools;
use rayon::prelude::*;
use std::iter::successors;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u64> {
    Some(secrets(input).into_par_iter().map(evolve2000).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let options: Vec<_> = secrets(input)
        .into_par_iter()
        .map(|secret| first_price_for_each_pattern(secret))
        .collect();

    let mut totals = HashMap::new();
    for h in options {
        for (pattern, price) in h {
            *totals.entry(pattern).or_insert(0) += price;
        }
    }
    Some(*totals.values().max().unwrap())
}

fn first_price_for_each_pattern(secret: u64) -> HashMap<(isize, isize, isize, isize), usize> {
    let mut seen = HashMap::new();
    for ((_, d1), (_, d2), (_, d3), (price, d4)) in evolve(secret, 2000)
        .map(|n| (n % 10) as isize)
        .tuple_windows()
        .map(|(a, b)| (b, b - a))
        .tuple_windows()
    {
        let pattern = (d1, d2, d3, d4);
        if !seen.contains_key(&pattern) {
            seen.insert(pattern, price as usize);
        }
    }
    seen
}

fn secrets(input: &str) -> Vec<u64> {
    parser!(lines(u64)).parse(input).unwrap()
}

fn evolve(secret: u64, n: usize) -> impl Iterator<Item = u64> + 'static {
    successors(Some(secret), |s| Some(evolve1(*s)))
        .skip(1)
        .take(n)
}

fn evolve2000(secret: u64) -> u64 {
    evolve(secret, 2000).last().unwrap()
}

fn evolve1(s0: u64) -> u64 {
    let s1 = (s0 ^ (s0 * 64)) % 16777216;
    let s2 = (s1 ^ (s1 / 32)) % 16777216;
    let s3 = (s2 ^ (s2 * 2048)) % 16777216;
    s3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
