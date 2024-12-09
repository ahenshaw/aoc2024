#![allow(unused_variables)]
#![warn(dead_code)]
use itertools::Itertools;
advent_of_code::solution!(9);

// use aoc_parse::{parser, prelude::*};
// use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let stream: Vec<_> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let mut disk: Vec<Option<usize>> = vec![];
    let mut id = 0;
    let mut first = true;
    for size in stream {
        for _ in 0..size {
            let content = if first { Some(id) } else { None };
            disk.push(content);
        }
        first = !first;
        if first {
            id += 1
        }
    }
    loop {
        let i = disk.iter().position(|&x| x.is_none()).unwrap();
        let j = disk.iter().rposition(|&x| !x.is_none()).unwrap();
        if i >= j {
            break;
        }
        disk.swap(i, j);
    }
    Some(
        disk.iter()
            .enumerate()
            .map(|(i, x)| i * x.unwrap_or(0))
            .sum(),
    )
}

#[derive(Debug)]
struct Block {
    data: Vec<Option<usize>>,
    used: usize,
}

impl Block {
    fn init(id: usize, used: usize, free: usize) -> Self {
        let mut data: Vec<Option<usize>> = Vec::new();
        for _ in 0..used {
            data.push(Some(id))
        }
        for _ in 0..free {
            data.push(None)
        }
        Self { data, used }
    }

    fn checksum(&self, start: usize) -> usize {
        self.data
            .iter()
            .enumerate()
            .map(|(i, x)| x.unwrap_or(0) * (i + start))
            .sum()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn blockmove(&mut self, other: &mut Self) -> bool {
        let need = other.used;
        if let Some((index, _)) = self
            .data
            .windows(need)
            .enumerate()
            .find(|(_, slice)| slice.iter().all(|x| x.is_none()))
        {
            self.data[index..index + need].swap_with_slice(&mut other.data[..need]);
            other.used = 0;
            return true;
        }
        false
    }
}

use std::fmt;
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in &self.data {
            match x {
                Some(x) => write!(f, "{}", x)?,
                None => write!(f, ".")?,
            }
        }
        write!(f, "|")?;
        Ok(())
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut stream: Vec<_> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    // make sure stream ends with a value for "free"
    if !stream.len() % 2 == 0 {
        stream.push(0);
    }
    dbg!(&stream.len());
    let mut disk: Vec<Block> = stream
        .iter()
        .tuples()
        .enumerate()
        .map(|(id, (used, free))| Block::init(id, *used, *free))
        .collect();
    for block in &disk {
        print!("{block}");
    }
    println!();
    println!();
    for src in (1..disk.len()).rev() {
        // println! {"{src}"};
        let (dsts, srcs) = disk.split_at_mut(src);
        for dst in 0..src - 1 {
            // println! {"    {dst}"};
            if dsts[dst].blockmove(&mut srcs[0]) {
                for block in dsts {
                    print!("{block}");
                }
                for block in srcs {
                    print!("{block}");
                }
                println!();
                break;
            }
        }
    }
    for block in &disk {
        println!("{block}");
    }
    let mut total = 0;
    let mut start = 0;
    for block in disk {
        total += block.checksum(start);
        start += block.len();
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
