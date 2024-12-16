#![allow(unused_variables)]
advent_of_code::solution!(15);

// use aoc_parse::{parser, prelude::*};
// use itertools::Itertools;
use advent_of_code::{Coord, Grid};

pub fn part_one(input: &str) -> Option<u32> {
    // let p = parser!(lines(repeat_sep(i32, " ")));
    // let Ok(lines) = p.parse(input) else {
    //     return None;
    // };

    let sections: Vec<&str> = input.split("\n\n").collect();
    let mut grid: Grid<char> = sections[0].chars().collect();
    println!("Initial state:");
    println!("{grid}");
    let (mut pos, _) = grid.iter().find(|(_, ch)| *ch == '@').unwrap();
    for movement in sections[1].replace("\n", "").chars() {
        pos = push(&mut grid, pos, movement);
        println!("{pos:?}");
        println!("\nMove {movement}:");
        println!("{grid}");
    }
    None
}

fn push(grid: &mut Grid<char>, pos: Coord, movement: char) -> Coord {
        use advent_of_code::Direction::*;
        let test = match movement {
            '^' => pos + North,
            'v' => pos + South,
            '>' => pos + East,
            '<' => pos + West,
            _ => unreachable!()
        };
        let target = grid.get(&test).unwrap();
        print!("Target: {target}  ");
        match target {
            '#' => return pos,
            '.' => {
                grid.container.insert(test, grid.get(&pos).unwrap().clone());
                grid.container.insert(pos, '.');
                return test;
            }
            'O' => {
                return push(grid, test, movement);
            }
            _ => return pos,
        }
}

pub fn part_two(input: &str) -> Option<u32> {
    // let p = parser!(lines(repeat_sep(i32, " ")));
    // let Ok(lines) = p.parse(input) else {
    //     return None;
    // };
    // let grid: Grid<char> = input.chars().collect();


    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
