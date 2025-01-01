#[allow(unused_imports)]
use advent_of_code::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

advent_of_code::solution!(9);

// First 9 triangular numbers (shifted by 1)
const TRI_NUMS: [usize; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];

pub fn part_one(input: &str) -> Option<usize> {
    let storage = parser!(line(digit*)).parse(input).unwrap();
    let mut checksum = Checksum::default();

    let mut left = 0;
    let mut right = (storage.len() + 1) / 2;
    let mut free_space = 0;
    let mut to_move = 0;

    while left < right {
        if free_space == 0 {
            checksum.update(storage[left << 1], left);
            free_space = storage[(left << 1) + 1];
            left += 1;
        } else if to_move == 0 {
            right -= 1;
            to_move = storage[right << 1];
        } else {
            let file_size = free_space.min(to_move);
            checksum.update(file_size, right);
            free_space -= file_size;
            to_move -= file_size;
        }
    }
    checksum.update(to_move, left);

    Some(checksum.value)
}

pub fn part_two(input: &str) -> Option<usize> {
    let storage = parser!(line(digit*)).parse(input).unwrap();
    let mut checksum = Checksum::default();

    let mut gaps: Vec<_> = (0..=9).map(|i| BinaryHeap::with_capacity(2_000)).collect();
    let mut file_address = 0;

    for (i, file_size) in storage.iter().enumerate() {
        if i % 2 == 1 && *file_size > 0 {
            gaps[*file_size].push(Reverse(file_address));
        }
        file_address += *file_size;
    }

    for (right, file_size) in storage.into_iter().enumerate().rev() {
        file_address -= file_size;

        if right % 2 == 1 {
            continue;
        }

        checksum.pos = file_address;
        let mut gap_size = None;
        for s in (file_size..gaps.len()) {
            if let Some(a) = gaps[s].peek() {
                if a.0 < checksum.pos {
                    checksum.pos = a.0;
                    gap_size = Some(s);
                } else if a.0 > file_address {
                    // Optimization - no reason to check gaps that occur AFTER
                    // the file_address we're currently at
                    gaps[s].clear();
                }
            }
        }

        // Optimization - clean up gap heaps that aren't needed
        while !gaps.is_empty() && gaps[gaps.len() - 1].is_empty() {
            gaps.pop();
        }

        checksum.update(file_size, right / 2);

        if let Some(s) = gap_size {
            gaps[s].pop();
            if s > file_size {
                gaps[s - file_size].push(Reverse(checksum.pos));
            }
        }
    }

    Some(checksum.value)
}

#[derive(Default)]
struct Checksum {
    value: usize,
    pos: usize,
}

impl Checksum {
    #[inline]
    fn update(&mut self, file_size: usize, file_id: usize) {
        self.value += (self.pos * file_size + TRI_NUMS[file_size]) * file_id;
        self.pos += file_size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
