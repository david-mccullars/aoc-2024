#[allow(unused_imports)]
use advent_of_code::*;
use core::option::Iter;
use std::collections::HashSet;
use std::iter::Filter;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    Some(PrintQueue::new(input).correct_print_updates())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(PrintQueue::new(input).fixed_print_updates())
}

#[derive(Debug)]
struct PrintQueue {
    page_ordering_rules: HashSet<(u32, u32)>,
    print_updates: Vec<Vec<u32>>,
}

impl PrintQueue {
    fn new(input: &str) -> Self {
        let (page_ordering_rules, print_updates) =
            parser!(section(hash_set(lines(u32 "|" u32))) section(lines(repeat_sep(u32, ","))))
                .parse(input)
                .unwrap();
        Self {
            page_ordering_rules,
            print_updates,
        }
    }

    fn correct_print_updates(&self) -> u32 {
        self.print_updates
            .iter()
            .filter(|pages| self.is_in_order(pages))
            .map(|x| middle(x))
            .sum()
    }

    fn fixed_print_updates(&self) -> u32 {
        self.print_updates
            .iter()
            .filter_map(|pages| self.fix(pages))
            .map(|x| middle(&x))
            .sum()
    }

    fn is_in_order(&self, pages: &[u32]) -> bool {
        for i2 in (1..pages.len()) {
            for i1 in (0..i2) {
                if self.page_ordering_rules.contains(&(pages[i2], pages[i1])) {
                    return false;
                }
            }
        }
        true
    }

    fn fix(&self, pages: &Vec<u32>) -> Option<Vec<u32>> {
        let mut pages = pages.clone();
        let mut fixed = false;
        for i2 in (1..pages.len()) {
            for i1 in (0..i2) {
                if self.page_ordering_rules.contains(&(pages[i2], pages[i1])) {
                    (pages[i2], pages[i1]) = (pages[i1], pages[i2]);
                    fixed = true;
                }
            }
        }
        fixed.then_some(pages)
    }
}

fn middle(pages: &[u32]) -> u32 {
    pages[pages.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
