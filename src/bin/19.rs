#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::HashMap;
use std::cmp::Ordering;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        Onsen::new(input)
            .possible_arrangement_counts()
            .filter(|n| *n > 0)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Onsen::new(input).possible_arrangement_counts().sum())
}

struct Onsen {
    towels: Vec<String>,
    arrangements: Vec<String>,
}

impl Onsen {
    fn new(input: &str) -> Self {
        let pattern = parser!(string(char_of("wubrg")+));
        let (towels, arrangements) = parser!(
            section(line(repeat_sep(pattern, ", ")))
            section(lines(pattern))
        )
        .parse(input)
        .unwrap();

        Self {
            towels,
            arrangements,
        }
    }

    fn possible_arrangement_counts(&self) -> impl Iterator<Item = usize> + '_ {
        self.arrangements
            .iter()
            .map(|arrangement| self.possible_arrangement_count(arrangement))
    }

    fn possible_arrangement_count(&self, arrangement: &str) -> usize {
        memoize(arrangement, || {
            self.towels
                .iter()
                .map(|towel| self.count(towel, arrangement))
                .sum()
        })
    }

    fn count(&self, towel: &str, arrangement: &str) -> usize {
        match towel.len().cmp(&arrangement.len()) {
            Ordering::Equal => {
                if towel == arrangement {
                    1
                } else {
                    0
                }
            }
            Ordering::Less => {
                if arrangement.starts_with(towel) {
                    self.possible_arrangement_count(&arrangement[towel.len()..])
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
}

// WARNING - this will bleed over between runs, so if you change
// the input the result will be wrong.
use std::sync::Mutex;
lazy_static::lazy_static! {
    static ref MEMO: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
}

fn memoize<F>(key: &str, f: F) -> usize
where
    F: Fn() -> usize,
{
    if let Some(&value) = MEMO.lock().unwrap().get(key) {
        return value;
    }

    let value = f();
    MEMO.lock().unwrap().insert(key.to_owned(), value);
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
