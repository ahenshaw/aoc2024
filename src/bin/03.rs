// use aoc_parse::{parser, prelude::*};
// use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").ok()?;
    let muls:Vec<_> = re.captures_iter(input)
        .map(|caps| {
            let (_, [a, b]) = caps.extract();
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        }).collect();
    Some(muls.iter().map(|(a, b)|  a * b).sum())

}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)|(?<do>do\(\))|(?<dont>don\'t\(\))").unwrap();
    let mut compute = true;
    let mut total = 0;
    for cap in re.captures_iter(input) {
        if cap.name("do").is_some() {compute = true}
        if cap.name("dont").is_some() {compute = false}
        if cap.name("a").is_some() && compute {
            let a = cap.name("a").unwrap().as_str().parse::<u32>().unwrap();
            let b = cap.name("b").unwrap().as_str().parse::<u32>().unwrap();
            total += a * b
        }

    }
    Some(total)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
