#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::HashMap;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    Some(WordSearch::new(input).count_any_xmas())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(WordSearch::new(input).count_diagonal_mas())
}

struct WordSearch {
    map: HashMap<Pos, char>,
}

impl WordSearch {
    fn new(input: &str) -> Self {
        let mut map = HashMap::new();
        for (y, row) in input.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                map.insert(pos_from(x, y), c);
            }
        }
        Self { map }
    }

    fn count_any_xmas(&self) -> usize {
        self.map
            .iter()
            .filter(|(_, c)| **c == 'X')
            .map(|(pos, _)| self.count_any_xmas_at(pos))
            .sum()
    }

    fn count_any_xmas_at(&self, pos: &Pos) -> usize {
        let mut found = 0;
        for dy in (-1..=1) {
            for dx in (-1..=1) {
                if self.char(pos) == 'X'
                    && self.char_near(pos, dx, dy) == 'M'
                    && self.char_near(pos, dx * 2, dy * 2) == 'A'
                    && self.char_near(pos, dx * 3, dy * 3) == 'S'
                {
                    found += 1;
                }
            }
        }
        found
    }

    fn count_diagonal_mas(&self) -> usize {
        self.map
            .iter()
            .filter(|(pos, c)| **c == 'A' && self.is_diagonal_mas_at(pos))
            .count()
    }

    fn is_diagonal_mas_at(&self, pos: &Pos) -> bool {
        let pair1 = (self.char_near(pos, -1, -1), self.char_near(pos, 1, 1));
        let pair2 = (self.char_near(pos, -1, 1), self.char_near(pos, 1, -1));

        (pair1 == ('M', 'S') || pair1 == ('S', 'M')) && (pair2 == ('M', 'S') || pair2 == ('S', 'M'))
    }

    fn char(&self, pos: &Pos) -> char {
        match self.map.get(pos) {
            Some(c) => *c,
            None => ' ',
        }
    }

    fn char_near(&self, pos: &Pos, dx: isize, dy: isize) -> char {
        self.char(&(pos.0 + dx, pos.1 + dy))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
