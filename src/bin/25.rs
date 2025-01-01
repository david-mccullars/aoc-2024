#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<usize> {
    let (locks, keys): (Vec<u64>, Vec<u64>) = input
        .replace("\n", "")
        .chars()
        .chunks(35)
        .into_iter()
        .map(|chunk| chunk.fold(0, |s, c| (s << 1) + if c == '#' { 1 } else { 0 }))
        .partition(|p| p & 1 == 0);

    Some(
        locks
            .into_iter()
            .cartesian_product(keys.into_iter())
            .filter(|(p1, p2)| p1 & p2 == 0)
            .count(),
    )
}

pub fn part_two(_input: &str) -> Option<&str> {
    Some("CLAIM THE FINAL GOLD STAR!!!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
