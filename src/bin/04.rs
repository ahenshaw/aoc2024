// use aoc_parse::{parser, prelude::*};
// use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    // let p = parser!(lines(repeat_sep(i32, " ")));
    // let Ok(lines) = p.parse(input) else {
    //     return None;
    // };
    dbg!(&input);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    // let p = parser!(lines(repeat_sep(i32, " ")));
    // let Ok(lines) = p.parse(input) else {
    //     return None;
    // };
    dbg!(&input);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}