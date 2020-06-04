use crate::load_input;
use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

crate::aoc!(09);

pub fn day09_main() -> Result<()> {
    let input = load_input(9)?;
    let re = Regex::new(r"(\w+) to (\w+) = (\d+)")?;

    let mut graph = HashMap::new();

    // this is basically solving the travelling salesperson problem
    // I really hate this solution. There's about twelve dozen unnecessary heap allocations,
    // I'm manually iterating over the adjacency list for each path element, I didn't even
    // implement the permutations myself

    for cap in re.captures_iter(&input) {
        let dist = cap[3].parse::<u32>()?;
        graph
            .entry(cap[1].to_owned())
            .and_modify(|x: &mut Vec<_>| x.push((cap[2].to_owned(), dist)))
            .or_insert(vec![(cap[2].to_owned(), dist)]);
        graph
            .entry(cap[2].to_owned())
            .and_modify(|x: &mut Vec<_>| x.push((cap[1].to_owned(), dist)))
            .or_insert(vec![(cap[1].to_owned(), dist)]);
    }

    assert_eq!(251, part1_solution(&graph));
    assert_eq!(898, part2_solution(&graph));

    Ok(())
}

fn part1_solution(graph: &HashMap<String, Vec<(String, u32)>>) -> u32 {
    let mut min_len = u32::MAX;
    for path in graph.keys().permutations(graph.len()) {
        let length = path
            .iter()
            .tuple_windows::<(_, _)>()
            .map(|(a, b)| graph[*a].iter().find(|x| x.0 == **b).unwrap().1)
            .sum();
        if length < min_len {
            min_len = length;
        }
    }
    min_len
}

fn part2_solution(graph: &HashMap<String, Vec<(String, u32)>>) -> u32 {
    let mut max_len = u32::MIN;
    for path in graph.keys().permutations(graph.len()) {
        let length = path
            .iter()
            .tuple_windows::<(_, _)>()
            .map(|(a, b)| graph[*a].iter().find(|x| x.0 == **b).unwrap().1)
            .sum();
        if length > max_len {
            max_len = length;
        }
    }
    max_len
}
