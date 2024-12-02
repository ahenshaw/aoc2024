use aoc_parse::{parser, prelude::*};
use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let p = parser!(lines(repeat_sep(i32, " ")));
    let Ok(lines) = p.parse(input) else {
        return None;
    };

    Some(lines.iter().filter(|line| is_safe(line)).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let p = parser!(lines(repeat_sep(i32, " ")));
    let Ok(lines) = p.parse(input) else {
        return None;
    };

    Some(
        lines
            .iter()
            .filter(|&line| {
                (1..=line.len()).any(|i| {
                    let dampened = &[&line[0..i - 1], &line[i..]].concat();
                    is_safe(dampened)
                })
            })
            .count(),
    )
}

fn is_safe(line: &Vec<i32>) -> bool {
    let delta: Vec<i32> = line.iter().tuple_windows().map(|(a, b)| a - b).collect();
    delta.iter().map(|x| x.abs()).all(|x| (1..=3).contains(&x))
        && (delta.iter().all(|&x| x > 0) || delta.iter().all(|&x| x < 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
