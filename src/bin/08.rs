advent_of_code::solution!(8);

use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use advent_of_code::{Grid, Coord};


pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<char> = input.chars().collect();
    let mut antinodes = HashSet::<Coord>::new();
    let mut antennas = HashMap::<char, Vec<Coord>>::new();

    for y in 0..grid.height as isize {
        for x in 0..grid.width as isize {
            match grid.get(x, y) {
                Some('.') => (),
                Some(ch) => {
                    antennas.entry(*ch).or_insert_with(Vec::new).push(Coord{x, y});
                },
                None => unreachable!(),
            }
        }
    }
    for (_, coords) in antennas {
        for (a, b) in coords.iter().tuple_combinations() {
            let dx = a.x - b.x;
            let dy = a.y - b.y;

            let x = a.x + dx;
            let y = a.y + dy;
            if grid.is_in(x, y){
                antinodes.insert(Coord{x, y});
            }
            let x = b.x - dx;
            let y = b.y - dy;
            if grid.is_in(x, y){
                antinodes.insert(Coord{x, y});
            }
        }
    }
    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Grid<char> = input.chars().collect();
    let mut antinodes = HashSet::<Coord>::new();
    let mut antennas = HashMap::<char, Vec<Coord>>::new();

    for y in 0..grid.height as isize {
        for x in 0..grid.width as isize {
            match grid.get(x, y) {
                Some('.') => (),
                Some(ch) => {
                    antennas.entry(*ch).or_insert_with(Vec::new).push(Coord{x, y});
                },
                None => unreachable!(),
            }
        }
    }
    for (_, coords) in antennas {
        for (a, b) in coords.iter().tuple_combinations() {
            let dx = a.x - b.x;
            let dy = a.y - b.y;
            for dir in [-1, 1] {
                let mut x = a.x;
                let mut y = a.y;
                while grid.is_in(x, y) {
                    antinodes.insert(Coord{x, y});
                    x += dir * dx;
                    y += dir * dy;
                }
            }
        }
    }
    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULT1: usize = 14;
    const RESULT2: usize = 34;
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(RESULT1));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(RESULT2));
    }
}
