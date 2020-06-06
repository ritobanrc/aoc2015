use crate::load_input;
use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

crate::aoc!(13);

pub fn day13_main() -> Result<()> {
    let input = load_input(13)?;
    let re = Regex::new(
        r"(?P<person1>\w+) would (?P<change>gain|lose) (?P<amount>\d+) happiness units by sitting next to (?P<person2>\w+).",
    )?;

    let mut graph: HashMap<_, Vec<(String, i32)>> = HashMap::new();

    for caps in re.captures_iter(&input) {
        let happiness =
            caps["amount"].parse::<i32>()? * if &caps["change"] == "lose" { -1 } else { 1 };
        graph
            .entry(caps["person1"].to_owned())
            .and_modify(|vec| vec.push((caps["person2"].to_owned(), happiness)))
            .or_insert(vec![(caps["person2"].to_owned(), happiness)]);
    }

    assert_eq!(
        733,
        part1_solution(&graph).ok_or(anyhow!("Solution 1 failed"))?
    );
    assert_eq!(
        725,
        part2_solution(&graph).ok_or(anyhow!("Solution 2 failed"))?
    );

    Ok(())
}

fn part1_solution(graph: &HashMap<String, Vec<(String, i32)>>) -> Option<i32> {
    graph
        .keys()
        .permutations(graph.keys().count())
        .map(|arrangement| {
            let mut happiness = 0i32;
            for (pos, name) in arrangement.iter().enumerate() {
                let neighbor1 = arrangement[(pos + 1) % arrangement.len()];
                happiness += graph[*name]
                    .iter()
                    .find(|(name, _)| name == neighbor1)
                    .unwrap()
                    .1;
                let neighbor2 = arrangement[(arrangement.len() + pos - 1) % arrangement.len()];
                happiness += graph[*name]
                    .iter()
                    .find(|(name, _)| name == neighbor2)
                    .unwrap()
                    .1;
            }
            happiness
        })
        .max()
}

fn part2_solution(graph: &HashMap<String, Vec<(String, i32)>>) -> Option<i32> {
    let mut new_graph = graph.clone();
    new_graph.insert("Me".to_owned(), vec![]);
    for person in graph.keys() {
        new_graph.get_mut("Me")?.push((person.to_owned(), 0));
        new_graph.get_mut(person)?.push(("Me".to_owned(), 0));
    }
    part1_solution(&new_graph)
}
