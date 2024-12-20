#![allow(unused_variables)]
advent_of_code::solution!(20);

use advent_of_code::{Coord, Grid, CARDINAL};
use petgraph::algo::astar;
use petgraph::data::Build;
use petgraph::graphmap::GraphMap;
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use petgraph::Directed;

const MAX_WEIGHT:u64 = 10000000;
pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<char> = input.chars().collect();

    let (start, _) = grid.iter().find(|(_, ch)| *ch == 'S').unwrap();
    let (end, _) = grid.iter().find(|(_, ch)| *ch == 'E').unwrap();
    let mut graph = GraphMap::<Coord, u64, Directed>::new();
    for (pos, _) in grid.iter() {
        graph.add_node(pos);
    }
    for (pos_a, ch_a) in grid.iter() {
        for dir in CARDINAL {
            let pos_b = pos_a + dir;
            if let Some(ch_b) = grid.get(&pos_b) {
                let weight = match ch_b {
                    '#' => MAX_WEIGHT,
                    _ => 1,
                };
                graph.add_edge(pos_a, pos_b, weight);
            }
        }
    }
    let savings = cheat(graph, start, end);
    Some(savings.iter().filter(|&x| *x >= 100).count())
}

pub fn part_two(input: &str) -> Option<u32> {
    // let p = parser!(lines(repeat_sep(i32, " ")));
    // let Ok(lines) = p.parse(input) else {
    //     return None;
    // };
    // let grid: Grid<char> = input.chars().collect();


    None
}

fn cheat(graph: GraphMap<Coord, u64, Directed>, start: Coord, end: Coord) -> Vec<u64>{
    let  (base, _) = astar(&graph, start, |n|n==end, |e| *e.weight(), |_| 0).unwrap();
    println!("Base: {base}");
    let mut savings = vec![];
    let mut changed = graph.clone();
    for edge in graph.edge_references() {
        if *edge.weight() == MAX_WEIGHT {
            let src = edge.source();
            let tgt = edge.target();
            changed.update_edge(src, tgt, 1);
            let (cost, _) = astar(&changed, start, |n|n==end, |e| *e.weight(), |_| 0).unwrap();
            changed.update_edge(src, tgt, MAX_WEIGHT);
            if cost < base {
                // println!("{} {:?}->{:?}", base - cost, src, tgt);
                savings.push(base - cost);
            }
        }
    }
    savings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
