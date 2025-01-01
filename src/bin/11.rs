#[allow(unused_imports)]
use advent_of_code::*;
use memoize::memoize;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    Some(blink_all(input, 25))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(blink_all(input, 75))
}

fn blink_all(input: &str, times: usize) -> usize {
    parser!(line(repeat_sep(usize, " ")))
        .parse(input)
        .unwrap()
        .into_iter()
        .map(|n| blink_single(n, times))
        .sum()
}

#[memoize]
fn blink_single(n: usize, times: usize) -> usize {
    if times == 0 {
        return 1;
    }

    if n == 0 {
        blink_single(1, times - 1)
    } else if let Some((left, right)) = split_even(n) {
        blink_single(left, times - 1) + blink_single(right, times - 1)
    } else {
        blink_single(n * 2024, times - 1)
    }
}

fn split_even(n: usize) -> Option<(usize, usize)> {
    let mut num = n;
    let mut digits = 0;

    while num > 0 {
        num /= 10;
        digits += 1;
    }

    if digits % 2 != 0 {
        return None;
    }

    let divisor = 10_usize.pow(digits / 2 as u32);

    Some((n / divisor, n % divisor))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
