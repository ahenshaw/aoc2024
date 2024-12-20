#![allow(unused_variables)]
advent_of_code::solution!(20);

use std::path;

use advent_of_code::{Coord, Grid, CARDINAL};
use petgraph::algo::astar;
use petgraph::data::Build;
use petgraph::graphmap::GraphMap;
use petgraph::visit::EdgeRef;
use petgraph::Undirected;

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Grid<char> = input.chars().collect();

    let (start, _) = grid.iter().find(|(_, ch)| *ch == 'S').unwrap();
    let (end, _) = grid.iter().find(|(_, ch)| *ch == 'E').unwrap();
    let mut graph = GraphMap::<Coord, u64, Undirected>::new();
    for (pos, _) in grid.iter() {
        graph.add_node(pos);
    }
    for (pos_a, ch_a) in grid.iter() {
        for dir in CARDINAL {
            let pos_b = pos_a + dir;
            if let Some(ch_b) = grid.get(&pos_b) {
                let weight = match (ch_a, ch_b) {
                    ('#', _) => 10000000,
                    (_, '#') => 10000000,
                    _ => 1,
                };
                graph.add_edge(pos_a, pos_b, weight);
            }
        }
    }
    graph.update_edge(Coord{x:8, y:7}, Coord{x:8, y:8}, 1);
    graph.update_edge(Coord{x:8, y:8}, Coord{x:8, y:9}, 1);
    // dbg!(graph.edge_weight(Coord{x:8, y:8}, Coord{x:8, y:9}));
    if let Some((cost, _)) = astar(&graph, start, |n|n==end, |e| *e.weight(), |_| 0) {
        dbg!(cost);

    }
    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
