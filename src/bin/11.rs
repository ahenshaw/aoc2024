#![allow(unused_variables)]
advent_of_code::solution!(11);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<isize> {
    solve(input, 25)
}

pub fn part_two(input: &str) -> Option<isize> {
    solve(input, 75)
}

fn solve(input: &str, blinks: usize) -> Option<isize> {
    let mut stones: HashMap<u64, isize> = input
        .split_whitespace()
        .map(|stone| (stone.parse().unwrap(), 1))
        .collect();

    for _ in 0..blinks {
        let mut nextgen = HashMap::<u64, isize>::new();
        for (stone, count) in stones {
            let mut right = stone.to_string();
            if stone == 0 {
                nextgen.insert(1, count);
            } else if right.len() % 2 == 0 {
                let left = right.split_off(right.len() / 2);
                let lefty: u64 = left.parse().unwrap();
                let right: u64 = right.parse().unwrap();
                nextgen.insert(lefty, count);
                nextgen.insert(right, count);
            } else {
                nextgen.insert(stone * 2024, count);
            }
        }
        stones = nextgen;
    }
    Some(stones.values().sum())
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
