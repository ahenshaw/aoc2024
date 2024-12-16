#![allow(unused_variables)]
advent_of_code::solution!(14);

use aoc_parse::{parser, prelude::*};
// use itertools::Itertools;
// use advent_of_code::{Coord, Grid, CARDINAL};

struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

impl Robot {
    fn pos(&self, t: i32, width: i32, height: i32) -> (i32, i32) {
        let nx = (self.x + t * self.vx).rem_euclid(width) ;
        let ny = (self.y + t * self.vy).rem_euclid(height);
        (nx, ny)
    }
}

fn quad(x: i32, y:i32, width: i32, height: i32) -> usize {
    if x == width / 2 {return 4}
    if y == height / 2 {return 4}
    let mut quad = if y < height / 2 {0} else {2};
    quad += if x < width / 2 {0} else {1};
    quad
}

fn std_dev(pts: &[(i32, i32)]) -> i32 {
    // just work on x
    let x_mean:i32 = pts.iter().map(|(x, _)| x).sum::<i32>() / pts.len() as i32;
    let y_mean:i32 = pts.iter().map(|(_, y)| y).sum::<i32>() / pts.len() as i32;
    let dev = pts.iter().map(|(x, y)| (*x - x_mean).pow(2) + (*y - y_mean).pow(2)).sum();
    dev
}

pub fn part_one(input: &str) -> Option<u32> {
    let p = parser!(lines("p=" i32 "," i32 " v=" i32 "," i32));
    let lines = p.parse(input).unwrap();
    let mut quads = [0;5];
    for (x, y, vx, vy) in lines {
        let r = Robot{x, y, vx, vy};
        let (nx, ny) = r.pos(100, WIDTH, HEIGHT);

        quads[quad(nx, ny, WIDTH, HEIGHT)] += 1;
    }

    Some(quads[0..4].iter().product())
}

pub fn part_two(input: &str) -> Option<u32> {
    let p = parser!(lines("p=" i32 "," i32 " v=" i32 "," i32));
    let lines = p.parse(input).unwrap();
    let mut stats = vec![];
    for t in 0..1000000 {
        let mut points: Vec<(i32, i32)> = vec![];
        for &(x, y, vx, vy) in &lines {
            let r = Robot{x, y, vx, vy};
            let (nx, ny) = r.pos(t, WIDTH, HEIGHT);
            points.push((nx, ny));
       }
       stats.push(std_dev(&points));
    }
    let best = stats[1..].iter().min().unwrap();
    let index = stats.iter().position(|x| x == best);
    dbg!(index, best);

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
