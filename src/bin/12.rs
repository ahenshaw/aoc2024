#![allow(unused_variables)]
advent_of_code::solution!(12);

use std::collections::HashSet;

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


fn get_sides(r: &HashSet<Coord>) -> u64 {
    use Direction::*;
    let sides: usize = r.iter().map(|&pt| {
        CARDINAL.iter().filter(|&dir| r.contains(&(pt.clone()+dir.clone()))).count()

    }).sum();
    println!("{} {sides}", r.len());
    sides as u64
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Grid<char> = input.chars().collect();
    let mut assigned = HashSet::<Coord>::new();
    let mut regions: Vec<HashSet<Coord>> = vec![];
    let mut plants: Vec<char> = vec![];
    for (pt, plant) in grid.iter() {
        if assigned.contains(&pt) {
            continue;
        }
        assigned.insert(pt.clone());
        let mut region = HashSet::<Coord>::new();
        get_region(&grid, &mut assigned, &pt, plant, &mut region);
        regions.push(region);
        plants.push(plant);
    }
    println!("{plants:?}");
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
        assert_eq!(result, Some(1206));
    }
}
