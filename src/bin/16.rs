#![allow(unused_variables)]
advent_of_code::solution!(16);
use std::collections::HashMap;

// use aoc_parse::{parser, prelude::*};
// use itertools::Itertools;
use advent_of_code::{Coord, Grid, Direction};

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Grid<char> = input.chars().collect();
    let (start, _) = grid.iter().find(|(pos, ch)| *ch == 'S').unwrap();
    let mut scores = HashMap::<Coord, u64>::new();

    let mut best = u64::MAX;
    search(&grid, start, Direction::East, 0, &mut best, &mut scores);
    println!("Lowest score: {best}");
    Some(best)
}

pub fn part_two(input: &str) -> Option<u32> {
    88416
    None
}

fn search(grid: &Grid<char>, pos: Coord, dir: Direction, score: u64, best: &mut u64, scores: &mut HashMap<Coord, u64>) {
    if scores.contains_key(&pos) {
        if scores.get(&pos).unwrap() <= &score {
            return;
        }
    }
    scores.insert(pos, score);
    let mut grid = grid.clone();
    grid.container.insert(pos, 'x');
    for  rot in [0, 1, 3] {
        let cost = (rot % 2) * 1000;
        let mut dir = dir.clone();
        for _ in 0..rot {
            dir = dir.rot90();
        }
        let pos = pos + dir.clone();

        let score = score + cost + 1;
        match grid.get(&pos) {
            Some('.') => {
                if score < *best {
                    search(&grid, pos, dir, score, best, scores)
                }
            },
            Some('E') => {
                if score < *best {
                    *best = score;
                }
                println!("{grid}");
                println!("{score}");

            }
            _ => (),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
