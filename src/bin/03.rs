#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .into_iter()
            .map(|op| match op {
                Some(Op::Multiply(a, b)) => a * b,
                _ => 0,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut enabled = true;
    let mut sum = 0;
    for op in parse(input) {
        match op {
            Some(Op::Multiply(a, b)) => {
                if enabled {
                    sum += a * b;
                }
            }
            Some(Op::Enable) => {
                enabled = true;
            }
            Some(Op::Disable) => {
                enabled = false;
            }
            None => {}
        }
    }
    Some(sum)
}

#[derive(Debug)]
enum Op {
    Multiply(u64, u64),
    Enable,
    Disable,
}

fn parse(input: &str) -> Vec<Option<Op>> {
    parser!({
        "mul(" a:u64 "," b:u64 ")" => Some(Op::Multiply(a, b)),
        "do()" => Some(Op::Enable),
        "don't()" => Some(Op::Disable),
        any_char => None,
    }*)
    .parse(input)
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
