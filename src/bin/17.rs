#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
advent_of_code::solution!(17);

use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use std::collections::HashMap;
use num_derive::FromPrimitive;

#[derive(FromPrimitive, Debug, Clone, Copy)]
enum Opcode {
    adv = 0,
    bxl = 1,
    bst = 2,
    jnz = 3,
    bxc = 4,
    out = 5,
    bdv = 6,
    cdv = 7,
}

#[derive(Debug)]
struct Cpu {
    a: u32,
    b: u32,
    c: u32,
    program: Vec<(Opcode, u32)>,
    ip: usize,
}

impl Cpu {
    fn from_str(input: &str) -> Self {
        let p = parser!(
            section(hash_map(lines("Register " upper ": " u32)))
            section(line("Program: " repeat_sep(u32, ",")))
        );
        let (registers, program): (HashMap<char, u32>, Vec<u32>) = p.parse(input).unwrap();
        let program: Vec<(Opcode, u32)> = program.iter().tuples().map(|(op, operand)| (num::FromPrimitive::from_u32(*op).unwrap(), *operand)).collect();
        Self { a: *registers.get(&'A').unwrap(),
               b: *registers.get(&'B').unwrap(),
               c: *registers.get(&'C').unwrap(),
               program,
               ip: 0,
            }
    }

    fn run(&mut self) -> String {
        use Opcode::*;
        let mut output = Vec::<u32>::new();
        loop {
            if self.ip >= self.program.len() {break}
            let (op, operand) = self.program[self.ip];
            match op {
                adv => {self.a /= 2u32.pow(self.combo(operand)); self.ip += 1;},
                bxl => {self.b ^= operand; self.ip += 1;},
                bst => {self.b = self.combo(operand) % 8; self.ip += 1;},
                jnz => {if self.a == 0 {self.ip += 1} else {self.ip = operand as usize / 2}},
                bxc => {self.b = self.b ^ self.c; self.ip += 1;},
                out => {output.push(self.combo(operand) % 8); self.ip += 1;},
                bdv => {self.b /= 2u32.pow(self.combo(operand)); self.ip += 1;},
                cdv => {self.c /= 2u32.pow(self.combo(operand)); self.ip += 1;},
            }
            // println!("{}", self.ip);
            dbg!(&self);
        }

        output.iter().map(|x| x.to_string()).join(",")
    }

    fn combo(&self, operand: u32) -> u32 {
        match operand {
            0..4 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!()
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut cpu = Cpu::from_str(input);
    dbg!(&cpu);
    Some(cpu.run())
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
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
