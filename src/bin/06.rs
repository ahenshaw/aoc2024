use std::collections::HashSet;

// use aoc_parse::{parser, prelude::*};
// use itertools::Itertools;
use advent_of_code::{Coord, Grid};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<char> = input.chars().collect();
    let index = input
        .find(|c: char| ['^', '>', '<', 'v'].contains(&c))
        .unwrap();
    let y = (index / (grid.width + 1)) as isize;
    let x = (index % (grid.width + 1)) as isize;
    let (dx, dy) = match grid.get_xy(x, y).unwrap() {
        '^' => (0, -1),
        '>' => (1, 0),
        'v' => (0, 1),
        '<' => (-1, 0),
        _ => unreachable!(),
    };

    // loop {
    //     let here = Coord{x,y};
    //     seen.insert(here);
    //     match grid.get(x + dx, y + dy) {
    //         None => break,
    //         Some('#') => {
    //             (dx, dy) = match (dx, dy) {
    //                 (0, -1) => (1,0),
    //                 (1, 0) => (0, 1),
    //                 (0, 1) => (-1, 0),
    //                 (-1, 0) => (0, -1),
    //                 _ => unreachable!()
    //             }
    //         }
    //         _ => {x += dx; y += dy;}

    //     }
    // }
    match patrol(&grid, x, y, dx, dy) {
        Outcome::Exited(visited) => return Some(visited.len()),
        Outcome::Looping => return None,
    }
}

fn rotate(dx: isize, dy: isize) -> (isize, isize) {
    match (dx, dy) {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => unreachable!(),
    }
}

enum Outcome {
    Exited(Vec<(isize, isize)>),
    Looping,
}

fn patrol(grid: &Grid<char>, sx: isize, sy: isize, dx: isize, dy: isize) -> Outcome {
    let mut visited = HashSet::<(isize, isize, isize, isize)>::new();

    let mut x = sx;
    let mut y = sy;
    let mut dx = dx;
    let mut dy = dy;

    loop {
        visited.insert((x, y, dx, dy));
        match grid.get_xy(x + dx, y + dy) {
            None => {
                let cells: HashSet<(isize, isize)> =
                    visited.iter().map(|(x, y, _, _)| (*x, *y)).collect();
                return Outcome::Exited(cells.iter().map(|(x, y)| (*x, *y)).collect());
            }
            Some('#') => {
                (dx, dy) = rotate(dx, dy);
                if visited.contains(&(x, y, dx, dy)) {
                    return Outcome::Looping;
                }
            }
            _ => {
                x += dx;
                y += dy;
            }
        }
    }
}
pub fn part_two(input: &str) -> Option<usize> {
    let mut grid: Grid<char> = input.chars().collect();
    let index = input
        .find(|c: char| ['^', '>', '<', 'v'].contains(&c))
        .unwrap();
    let sy = (index / (grid.width + 1)) as isize;
    let sx = (index % (grid.width + 1)) as isize;
    let (dx, dy) = match grid.get_xy(sx, sy).unwrap() {
        '^' => (0, -1),
        '>' => (1, 0),
        'v' => (0, 1),
        '<' => (-1, 0),
        _ => unreachable!(),
    };
    let mut count = 0;

    let Outcome::Exited(visited) = patrol(&grid, sx, sy, dx, dy) else {
        return None;
    };

    for (x, y) in visited {
        if x != sx || y != sy {
            grid.container.insert(Coord { x, y }, '#');
            match patrol(&grid, sx, sy, dx, dy) {
                Outcome::Exited(_) => (),
                Outcome::Looping => count += 1,
            }
            grid.container.insert(Coord { x, y }, '.');
        }
    }
    Some(count)
}

// fn is_going_to_loop(grid: &Grid<char>, x: isize, y: isize, dx: isize, dy: isize) -> bool {
//     let mut x = x;
//     let mut y = y;
//     let mut dx = dx;
//     let mut dy = dy;
//     let mut grid = grid.clone();
//     let mut seen = HashSet::<(isize, isize, isize, isize)>::new();
//     grid.container.insert(Coord{x: x+dx, y: y+dy}, '#');
//     loop {
//         if seen.contains(&(x, y, dx, dy)) {
//             return true
//         }
//         seen.insert((x, y, dx, dy));
//         match grid.get(x + dx, y + dy) {
//             None => return false,
//             Some('#') => {
//                 (dx, dy) = rotate(dx, dy);
//             }
//             _ => {
//                 x += dx; y += dy;
//             }
//         }
//     }

// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
