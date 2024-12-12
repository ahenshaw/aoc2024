#![allow(unused_variables)]
advent_of_code::solution!(12);

use std::collections::HashSet;

// use aoc_parse::{parser, prelude::*};
// use itertools::Itertools;
use advent_of_code::{Coord, Direction, Grid, CARDINAL, COMPASS};
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Grid<char> = input.chars().collect();
    let mut assigned = HashSet::<Coord>::new();
    let mut regions: Vec<HashSet<Coord>> = vec![];
    for (pt, plant) in grid.iter() {
        if assigned.contains(&pt) { continue;}
        assigned.insert(pt.clone());
        let mut region = HashSet::<Coord>::new();
        get_region(&grid, &mut assigned, &pt, plant, &mut region);
        regions.push(region);
    }
    Some(regions.iter().map(|r| get_perimeter(r) * r.len() as u64).sum())
}

fn get_region(grid: &Grid<char>, assigned: &mut HashSet<Coord>, pt: &Coord, plant: char, region: &mut HashSet<Coord>) {
    region.insert(pt.clone());
    for dir in CARDINAL {
        let cursor = pt.clone() + dir;
        if assigned.contains(&cursor) {
            continue;
        }
        match grid.get(&cursor) {
            Some(&target) => {
                if plant == target && !region.contains(&cursor) {
                    assigned.insert(cursor.clone());
                    get_region(grid, assigned, &cursor, plant, region);
                }
            },
            None => (),
        }
    }

}

fn get_perimeter(region: &HashSet<Coord>) -> u64 {
    let mut perimeter = 0;
    for pt in region {
        for dir in CARDINAL {
            let test = pt.clone() + dir;
            if !region.contains(&test) {
                perimeter += 1;
            }
        }
    }
    perimeter
}

fn get_sides(region: &HashSet<Coord>) -> u64 {
    let mut sides = 0;

    if region.len() == 1 {return 4}

    let mut inflated = HashSet::<Coord>::new();
    for pt in region {
        inflated.insert(pt.clone());
        for dir in COMPASS {
            inflated.insert(pt.clone()+dir);
        }
    }
    let outline: HashSet<&Coord> = inflated.difference(region).collect();
    let start = outline.iter().next().unwrap().clone();
    let dir = CARDINAL.iter().find(|&&d| outline.contains(&(start.clone()+d))).unwrap().clone();
    let mut cursor = start.clone() + dir.clone();
    while cursor != start {
        if !outline.contains(&(cursor + dir)) {
            sides += 1;
            dir = rot(dir);
        }
    }

    // let mut dir = Direction::East;
    sides
}

fn rot(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
        _ => unreachable!()
    }
}
pub fn part_two(input: &str) -> Option<u64> {
    let grid: Grid<char> = input.chars().collect();
    let mut assigned = HashSet::<Coord>::new();
    let mut regions: Vec<HashSet<Coord>> = vec![];
    for (pt, plant) in grid.iter() {
        if assigned.contains(&pt) { continue;}
        assigned.insert(pt.clone());
        let mut region = HashSet::<Coord>::new();
        get_region(&grid, &mut assigned, &pt, plant, &mut region);
        regions.push(region);
    }
    Some(regions.iter().map(|r| get_sides(r) * r.len() as u64).sum())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }
}
