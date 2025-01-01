#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<usize> {
    Some(Onsen::new(input).possibles().filter(|n| *n > 0).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Onsen::new(input).possibles().sum())
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

    fn possibles(&self) -> impl Iterator<Item = usize> + '_ {
        self.arrangements
            .iter()
            .map(|arrangement| self.possible(arrangement))
    }

    fn possible(&self, arrangement: &str) -> usize {
        memoize(arrangement, || {
            self.towels
                .iter()
                .map(|towel| {
                    if towel == arrangement {
                        1
                    } else if arrangement.starts_with(towel) {
                        self.possible(&arrangement[towel.len()..])
                    } else {
                        0
                    }
                })
                .sum()
        })
    }
}

// WARNING - this will bleed over between runs, so if you change
// the input the result will be wrong.
use hashbrown::HashMap;
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
