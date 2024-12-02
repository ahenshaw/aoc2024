advent_of_code::solution!(1);
use aoc_parse::{parser, prelude::*};

use counter::Counter;

pub fn part_one(input: &str) -> Option<u32> {
    let p = parser!(lines(u32 "   "  u32));
    let lines = p.parse(input).unwrap();

    let mut left = Vec::new();
    let mut right = Vec::new();

    for (a, b) in lines {
        left.push(a);
        right.push(b);
    }
    left.sort();
    right.sort();
    Some(left.iter().zip(right).map(|(a, b)| a.abs_diff(b)).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let p = parser!(lines(usize "   "  usize));

    let mut left = Vec::new();
    let mut right = Counter::<usize>::new();

    for (a, b) in p.parse(input).unwrap() {
        left.push(a);
        right[&b] += 1;
    }

    Some(left.iter().map(|a| a * right.get(a).unwrap_or(&0)).sum())
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
