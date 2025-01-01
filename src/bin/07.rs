#[allow(unused_imports)]
use advent_of_code::*;
use rayon::prelude::*;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<i64> {
    Some(total_calibration_result(input, false))
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(total_calibration_result(input, true))
}

fn total_calibration_result(input: &str, allow_concat: bool) -> i64 {
    parser!(lines(i64 ": " repeat_sep(i64, " ")))
        .parse(input)
        .unwrap()
        .par_iter()
        .filter(|eq| is_true(&eq.0, &eq.1, allow_concat))
        .map(|eq| eq.0)
        .sum()
}

fn is_true(total: &i64, nums: &[i64], allow_concat: bool) -> bool {
    let mut stack = vec![(*total, nums)];
    while let Some((t, n)) = stack.pop() {
        if n.len() == 1 {
            if n[0] == t {
                return true;
            }
        } else {
            let n_before = &n[0..n.len() - 1];
            let n_last = n[n.len() - 1];
            if t >= n_last {
                stack.push((t - n_last, n_before));
            }
            if let Some(t2) = unfactor(t, n_last) {
                stack.push((t2, n_before));
            }
            if allow_concat {
                if let Some(t2) = unconcat(t, n_last) {
                    stack.push((t2, n_before));
                }
            }
        }
    }
    false
}

fn unfactor(total: i64, num: i64) -> Option<i64> {
    let t2 = total / num;
    (t2 * num == total).then_some(t2)
}

fn unconcat(total: i64, num: i64) -> Option<i64> {
    if num == 0 {
        return (total % 10 == 0).then_some(total / 10);
    }
    let mut total = total;
    let mut num = num;
    while num > 0 {
        if total % 10 != num % 10 {
            return None;
        }
        total /= 10;
        num /= 10;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
