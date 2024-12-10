#![allow(unused_variables)]
advent_of_code::solution!(10);

use std::collections::HashSet;

use advent_of_code::{Coord, Grid, CARDINAL};

fn search(grid: &Grid<char>, here: Coord, val: char, dests: &mut HashSet<Coord>) {
    for dir in CARDINAL {
        let check = here.clone() + dir;
        if let Some(&ch) = grid.get(&check) {
            if ch as i8 - val as i8 == 1 {
                if ch == '9' {
                    dests.insert(check);
                    continue;
                } else {
                    search(grid, check, ch, dests);
                }
            }
        }
    }
}

fn search2(grid: &Grid<char>, here: Coord, val: char, paths: &mut usize) {
    for dir in CARDINAL {
        let check = here.clone() + dir;
        if let Some(&ch) = grid.get(&check) {
            if ch as i8 - val as i8 == 1 {
                if ch == '9' {
                    *paths += 1;
                    continue;
                } else {
                    search2(grid, check, ch, paths);
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<char> = input.chars().collect();
    let mut total = 0;
    for (here, ch) in grid.iter() {
        if ch == '0' {
            let mut dests = HashSet::<Coord>::new();
            search(&grid, here, '0', &mut dests);
            total += dests.len();
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Grid<char> = input.chars().collect();
    let mut total = 0;
    for (here, ch) in grid.iter() {
        if ch == '0' {
            search2(&grid, here, '0', &mut total);
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
