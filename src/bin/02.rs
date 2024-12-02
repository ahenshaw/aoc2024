use aoc_parse::{parser, prelude::*};
use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let p = parser!(lines(repeat_sep(i32, " ")));
    let mut count = 0u32;
    for line in p.parse(input).unwrap() {
        if is_safe(&line) {
            count += 1;
        }
    }

    Some(count)
}

fn is_safe(line: &Vec<i32>) -> bool {
    let delta: Vec<i32> = line.iter().tuple_windows().map(|(a,b)| a - b).collect();
    if delta[0] < 0 {
        if delta.iter().any(|&x| x >= 0) {return false};
    }else {
        if delta.iter().any(|&x| x < 0) {return false};
    }
    if delta.iter().any(|&x| x.abs()<1 || x.abs()>3) {return false};
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let p = parser!(lines(repeat_sep(i32, " ")));
    let mut count = 0u32;
    for line in p.parse(input).unwrap() {
        if is_safe(&line) {
            count += 1;
            continue;
        }
        for i in 1..line.len()+1 {
            let dampener = [&line[0..i-1], &line[i..]].concat();
            if is_safe(&dampener) {
                count += 1;
                break;
            }
        }
    }

    Some(count)

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
