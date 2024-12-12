#![allow(unused_variables)]
advent_of_code::solution!(12);

use std::collections::HashSet;

// use aoc_parse::{parser, prelude::*};
// use itertools::Itertools;
use advent_of_code::{Coord, Direction, Grid, CARDINAL, COMPASS};

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Grid<char> = input.chars().collect();
    let mut assigned = HashSet::<Coord>::new();
    let mut regions: Vec<HashSet<Coord>> = vec![];
    for (pt, plant) in grid.iter() {
        if assigned.contains(&pt) {
            continue;
        }
        assigned.insert(pt.clone());
        let mut region = HashSet::<Coord>::new();
        get_region(&grid, &mut assigned, &pt, plant, &mut region);
        regions.push(region);
    }
    Some(
        regions
            .iter()
            .map(|r| get_perimeter(r) * r.len() as u64)
            .sum(),
    )
}

fn get_region(
    grid: &Grid<char>,
    assigned: &mut HashSet<Coord>,
    pt: &Coord,
    plant: char,
    region: &mut HashSet<Coord>,
) {
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
            }
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
    println!("region: {region:?}");

    if region.len() == 1 {
        return 4;
    }

    let mut inflated = HashSet::<Coord>::new();
    for pt in region {
        inflated.insert(pt.clone());
        for dir in COMPASS {
            inflated.insert(pt.clone() + dir);
        }
    }
    let outline: HashSet<&Coord> = inflated.difference(region).collect();
    println!("outline: {outline:?}");

    let test = *(outline.iter().next().unwrap());
    let mut dir = CARDINAL
        .iter()
        .find(|&d| outline.contains(&(test.clone() + d.clone())))
        .unwrap()
        .clone();
    let start = test.clone() + dir.clone();
    let mut cursor = start.clone();
    println!("start: {start:?}   dir: {dir:?}");
    loop {
        println!("    cursor: {cursor:?}  sides: {sides}");
        if !outline.contains(&(cursor + dir.clone())) {
            dir = dir
                .orthogonal()
                .iter()
                .find(|&d| outline.contains(&(cursor.clone() + d.clone())))
                .unwrap()
                .clone();
            sides += 1;
            if sides > 100 {
                break;
            }
        }
        cursor = cursor + dir.clone();
        if cursor == start {
            break;
        }
    }

    // let mut dir = Direction::East;
    sides
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Grid<char> = input.chars().collect();
    let mut assigned = HashSet::<Coord>::new();
    let mut regions: Vec<HashSet<Coord>> = vec![];
    for (pt, plant) in grid.iter() {
        if assigned.contains(&pt) {
            continue;
        }
        assigned.insert(pt.clone());
        let mut region = HashSet::<Coord>::new();
        get_region(&grid, &mut assigned, &pt, plant, &mut region);
        regions.push(region);
    }
    regions.first().map(|r| get_sides(r));
    // Some(regions.iter().map(|r| get_sides(r) * r.len() as u64).sum())
    None
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
