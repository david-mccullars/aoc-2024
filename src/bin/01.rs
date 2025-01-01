#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::HashMap;
use std::cmp::min;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut lista: Vec<i32> = vec![];
    let mut listb: Vec<i32> = vec![];
    for (a, b) in parse(input) {
        lista.push(a);
        listb.push(b);
    }
    lista.sort();
    listb.sort();

    Some(
        lista
            .into_iter()
            .zip(listb)
            .into_iter()
            .fold(0, |sum, (a, b): (i32, i32)| sum + (a - b).abs()),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut lista: HashMap<i32, i32> = HashMap::new();
    let mut listb: HashMap<i32, i32> = HashMap::new();
    for (a, b) in parse(input) {
        *lista.entry(a).or_insert(0) += 1;
        *listb.entry(b).or_insert(0) += 1;
    }
    Some(lista.keys().fold(0, |sum, v| {
        let qtya: &i32 = lista.get(v).unwrap_or(&0);
        let qtyb: &i32 = listb.get(v).unwrap_or(&0);
        sum + v * qtya * qtyb
    }))
}

fn parse(input: &str) -> Vec<(i32, i32)> {
    parser!(lines(a:i32 " "+ b:i32 => (a, b)))
        .parse(input)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
