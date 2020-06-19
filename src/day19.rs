use crate::load_input;
use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::HashSet;

crate::aoc!(19);

pub fn day19_main() -> Result<()> {
    let input = load_input(19)?;
    let medicine = input
        .lines()
        .last()
        .ok_or(anyhow!("Didn't find medicine"))?;
    let molecule = separate_compound(medicine);
    let replacement_re = Regex::new(r"(?P<alpha>\w+) => (?P<beta>\w+)")?;
    let rules: Vec<_> = replacement_re
        .captures_iter(&input)
        .map(|caps| (caps["alpha"].to_string(), separate_compound(&caps["beta"])))
        .collect();

    assert_eq!(509, part1_solution(&molecule, &rules));

    // for part 2, it's easier for the rules to not have been split so that we can use string
    // replacement functions more easily
    let rules: Vec<_> = replacement_re
        .captures_iter(&input)
        .map(|caps| (caps["alpha"].to_string(), caps["beta"].to_string()))
        .collect();

    assert_eq!(195, part2_solution(medicine, &rules));

    Ok(())
}

fn separate_compound(s: &str) -> Vec<String> {
    let mut current_element_start = 0;
    let mut compound = Vec::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if current_element_start != i {
                compound.push(s[current_element_start..i].to_string());
            }
            current_element_start = i;
        }
    }
    compound.push(s[current_element_start..].to_string());
    compound
}

fn part1_solution(medicine: &[String], rules: &[(String, Vec<String>)]) -> usize {
    let mut compounds = HashSet::new();
    for (i, element) in medicine.iter().enumerate() {
        let replacements: Vec<_> = rules
            .iter()
            .filter(|(reactant, _)| reactant == element)
            .collect();
        for r in replacements {
            let mut new_compound = medicine[..i].to_vec();
            new_compound.extend_from_slice(&r.1);
            new_compound.extend_from_slice(&medicine[i + 1..]);
            compounds.insert(new_compound);
        }
    }
    compounds.len()
}

fn part2_solution(medicine: &str, rules: &[(String, String)]) -> usize {
    // Credit to u/askalski for realizing that you can travel backwards and not have to do any
    // backtracking
    // And credit to u/semi225599 for providing the python code that helped by understand what
    // was going on
    let mut medicine = medicine.to_owned();
    let mut count = 0;
    while medicine != "e" {
        // try every replacement and find the rightmost one
        let mut best_idx = 0;
        let (mut best_input, mut best_output) = (None, None);

        for (input, output) in rules {
            if let Some(idx) = medicine.rfind(output) {
                if idx >= best_idx {
                    best_idx = idx;
                    best_input = Some(input);
                    best_output = Some(output);
                }
            }
        }

        if let (Some(input), Some(output)) = (best_input, best_output) {
            // do the replacement
            let next_medicine = format!(
                "{}{}{}",
                &medicine[..best_idx],
                input,
                &medicine[best_idx + output.len()..],
            );
            count += 1;
            medicine = next_medicine;
        }
    }
    count
}
