#![allow(unused_variables)]
advent_of_code::solution!(10);

use std::collections::HashSet;

// use aoc_parse::{parser, prelude::*};
// use itertools::Itertools;
use advent_of_code::Grid;

fn search(grid: &Grid<char>, x: isize, y: isize, val: char, level: usize, paths:&mut HashSet<(isize, isize)>)  {
    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let x = x + dx;
        let y = y + dy;
        if let Some(&ch) = grid.get(x, y) {
            if ch as i8 - val as i8 == 1 {
                if ch == '9' {
                    paths.insert((x, y));
                    continue;
                } else {
                    search(grid, x, y, ch, level + 1, paths);
                }
            }
        }
    }

}

fn search2(grid: &Grid<char>, x: isize, y: isize, val: char, level: usize, paths:&mut usize)  {
    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let x = x + dx;
        let y = y + dy;
        if let Some(&ch) = grid.get(x, y) {
            if ch as i8 - val as i8 == 1 {
                if ch == '9' {
                    *paths += 1;
                    continue;
                } else {
                    search2(grid, x, y, ch, level + 1, paths);
                }
            }
        }
    }

}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<char> = input.chars().collect();
    let mut total = 0;
    for y in 0..grid.height as isize {
        for x in 0..grid.width as isize{
            if grid.get(x, y) == Some(&'0') {
                let mut paths: HashSet<(isize, isize)> = HashSet::new();
                search(&grid, x, y, '0', 0, &mut paths);
                total += paths.len();
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Grid<char> = input.chars().collect();
    let mut total = 0;
    for y in 0..grid.height as isize {
        for x in 0..grid.width as isize{
            if grid.get(x, y) == Some(&'0') {
                search2(&grid, x, y, '0', 0, &mut total);
            }
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
