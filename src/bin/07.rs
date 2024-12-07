use aoc_parse::{parser, prelude::*};
use rayon::prelude::*;

advent_of_code::solution!(7);

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    let p = parser!(lines(u64 ":" " " repeat_sep(u64, " ")));
    p.parse(input).unwrap()
}


fn eval(target: u64, total: u64, operands: &[u64], operators: &[char]) -> bool {
    if total > target {return false}

    let operand = operands[0];
    for operator in operators {
        let new_total = match operator {
            '+' => total + operand,
            '*' => total * operand,
            '|' => format!("{total}{operand}").parse().unwrap(),
            _ => unreachable!()
        };
        if operands.len() > 1 {
            if eval(target, new_total, &operands[1..], operators) {
                return true;
            }
        } else {
            if target == new_total {
                return true;
            }
        }
    }
    false
}
pub fn part_one(input: &str) -> Option<u64> {
    Some(parse_input(input)
    .par_iter()
    .filter(|(target, operands)| eval(*target, operands[0], &operands[1..], &['+', '*']))
    .map(|(target,_)| target)
    .sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(parse_input(input)
    .par_iter()
    .filter(|(target, operands)| eval(*target, operands[0], &operands[1..], &['+', '*', '|']))
    .map(|(target,_)| target)
    .sum())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
