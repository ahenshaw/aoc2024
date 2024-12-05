use std::collections::HashSet;

use aoc_parse::{parser, prelude::*};
use itertools::Itertools;

advent_of_code::solution!(5);

type Rules = HashSet<(u32, u32)>;
type Pages = Vec<u32>;

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, production) = parse_input(input);

    Some(production.iter()
    .filter(|pages| is_valid(&rules, pages))
    .map(|pages| pages[pages.len()/2])
    .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, production) = parse_input(input);

    Some(production.iter()
    .filter(|pages| !is_valid(&rules, pages))
    .map(|pages| {
        let fixed = fix_order(&rules, &pages);
        fixed[fixed.len()/2]
    })
    .sum())

}

fn parse_input(input: &str) -> (Rules, Vec<Pages>) {
    let p = parser!(
        section(lines(u32 "|" u32))
        section(lines(repeat_sep(u32, ",")))
    );
    let (rules, production) = p.parse(input).unwrap();
    let rules: Rules = rules.into_iter().collect();
    (rules, production)
}

fn is_valid(rules: &Rules, pages: &Pages) -> bool {
    pages.iter().tuple_combinations().all(|(a, b)| rules.contains(&(*a, *b)))
}

fn fix_order(rules: &Rules, pages: &Pages) -> Pages {
    let mut pages = pages.clone();

    loop{
        for (i, j) in (0..pages.len()).tuple_combinations() {
            if !rules.contains(&(pages[i], pages[j])) {
                pages.swap(i, j);
            }
        }
        if is_valid(rules, &pages) {
            break;
        }
    }
    pages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
