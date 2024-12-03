#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;
use std::ops::RangeInclusive;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    Some(count_safe(input, is_safe))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(count_safe(input, is_safe_with_one_removed))
}

fn count_safe(input: &str, safety_check: fn(&[i32]) -> bool) -> usize {
    parser!(lines(repeat_sep(i32, " ")))
        .parse(input)
        .unwrap()
        .into_iter()
        .filter(|nums| safety_check(nums))
        .count()
}

fn is_safe(nums: &[i32]) -> bool {
    let diffs: Vec<i32> = nums
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| a - b)
        .collect();
    diffs.iter().all(|v| (1..=3).contains(v)) || diffs.iter().all(|v| (-3..=-1).contains(v))
}

fn is_safe_with_one_removed(nums: &[i32]) -> bool {
    let mut one_removed: Vec<i32> = nums[1..].to_vec();
    for i in (0..nums.len()) {
        if i > 0 {
            one_removed[i - 1] = nums[i - 1];
        }
        if is_safe(&one_removed) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
