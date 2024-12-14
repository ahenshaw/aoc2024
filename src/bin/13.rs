#![allow(unused_variables)]
advent_of_code::solution!(13);

use aoc_parse::{parser, prelude::*};

pub fn part_one(input: &str) -> Option<i64> {
    let mut total = 0;
    let button_parser = parser!(
        line("Button " u8_hex ": X+" i64 ", Y+" i64)
    );
    let prize_parser = parser!(
        line("Prize: X=" i64 ", Y=" i64)
    );
    let mut buttons: Vec<(i64, i64)> = vec![];
    for line in input.lines() {
        if line.starts_with("B") {
            let (_, dx, dy) = button_parser.parse(line).unwrap();
            buttons.push((dx, dy))
        } else if line.starts_with("P") {
            let (tx, ty) = prize_parser.parse(line).unwrap();

            total += solve(&[tx, ty], buttons);
            buttons = vec![];
        } else {
            continue
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut total = 0;
    let button_parser = parser!(
        line("Button " u8_hex ": X+" i64 ", Y+" i64)
    );
    let prize_parser = parser!(
        line("Prize: X=" i64 ", Y=" i64)
    );
    let mut buttons: Vec<(i64, i64)> = vec![];
    for line in input.lines() {
        if line.starts_with("B") {
            let (_, dx, dy) = button_parser.parse(line).unwrap();
            buttons.push((dx, dy))
        } else if line.starts_with("P") {
            let (tx, ty) = prize_parser.parse(line).unwrap();
            let tx = tx + 10000000000000;
            let ty = ty + 10000000000000;

            total += solve(&[tx, ty], buttons);
            buttons = vec![];
        } else {continue}
    }
    Some(total)
}

fn solve(prize:&[i64], btns:Vec<(i64, i64)>) -> i64 {
    let a1 = btns[0].0;
    let a2 = btns[0].1;
    let b1 = btns[1].0;
    let b2 = btns[1].1;
    let left = a1*b2 - b1*a2;
    let right = b2*prize[0] - b1*prize[1];
    let (a, rem) = (right / left, right % left);
    if rem == 0 {
        let eqn = prize[0] - a * a1;
        let (b, rem) = (eqn / b1, eqn % b1);
        if rem == 0 {
            return 3*a + b
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
