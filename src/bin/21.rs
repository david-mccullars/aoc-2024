#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::HashMap;
use itertools::Itertools;
use memoize::memoize;
use std::iter::{once, repeat};

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<usize> {
    Some(key_cost_checksum(input, 3))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(key_cost_checksum(input, 26))
}

const ROW_SIZE: usize = 3;

type KeyPress = (char, char);

fn key_cost_checksum(codes: &str, layers: usize) -> usize {
    codes
        .lines()
        .map(|code| {
            let n = code[0..code.len() - 1].parse::<usize>().unwrap();
            n * key_cost(code.to_owned(), layers)
        })
        .sum()
}

#[memoize]
fn key_cost(code: String, layers: usize) -> usize {
    if layers == 0 {
        code.len()
    } else {
        once('A')
            .chain(code.chars())
            .tuple_windows()
            .map(|pair: KeyPress| {
                MOVEMENT_OPTIONS
                    .get(&pair)
                    .unwrap()
                    .iter()
                    .map(|subcode| key_cost(subcode.clone(), layers - 1))
                    .min()
                    .unwrap()
            })
            .sum()
    }
}

lazy_static::lazy_static! {
    static ref MOVEMENT_OPTIONS: HashMap<KeyPress, Vec<String>> = movement_options(vec![" ^A<v>", "789456123 0A"]);
}

fn movement_options(key_sets: Vec<&str>) -> HashMap<KeyPress, Vec<String>> {
    let mut options = HashMap::new();
    for keys in key_sets {
        let gap = keys.chars().position(|c| c == ' ').unwrap();
        let it = keys.chars().enumerate();
        for ((i1, c1), (i2, c2)) in it.clone().cartesian_product(it) {
            if c1 != ' ' && c2 != ' ' {
                options.insert((c1, c2), options_between(i1, i2, gap));
            }
        }
    }
    options
}

fn options_between(i1: usize, i2: usize, ig: usize) -> Vec<String> {
    let xy1 = (i1 % ROW_SIZE, i1 / ROW_SIZE);
    let xy2 = (i2 % ROW_SIZE, i2 / ROW_SIZE);
    let gap = (ig % ROW_SIZE, ig / ROW_SIZE);

    let (cx, dx) = if xy2.0 > xy1.0 {
        ('>', xy2.0 - xy1.0)
    } else {
        ('<', xy1.0 - xy2.0)
    };
    let (cy, dy) = if xy2.1 > xy1.1 {
        ('v', xy2.1 - xy1.1)
    } else {
        ('^', xy1.1 - xy2.1)
    };
    let xy = repeat(cx)
        .take(dx)
        .chain(repeat(cy).take(dy))
        .chain(once('A'));
    let yx = repeat(cy)
        .take(dy)
        .chain(repeat(cx).take(dx))
        .chain(once('A'));

    if xy2.0 == gap.0 && xy1.1 == gap.1 {
        vec![yx.collect()]
    } else if xy1.0 == xy2.0 || xy1.1 == xy2.1 || (xy1.0 == gap.0 && xy2.1 == gap.1) {
        vec![xy.collect()]
    } else {
        vec![xy.collect(), yx.collect()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
