use crate::load_input;
use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::HashMap;

crate::aoc!(07);
pub fn day07_main() -> Result<()> {
    let input = load_input(7)?;
    let circuit = {
        let mut circuit = HashMap::new();
        for line in input.lines() {
            let (output, expr) = line
                .rsplit(" -> ")
                .collect_tuple::<(_, _)>()
                .ok_or(anyhow!("Failed to split line on ->. Line: {:?}", line))?;
            circuit.insert(output, expr);
        }
        circuit
    };

    let p1 = part1_solution(&circuit)?;
    assert_eq!(956, p1);
    assert_eq!(40149, part2_solution(&circuit, p1)?);

    Ok(())
}

fn part1_solution(circuit: &HashMap<&str, &str>) -> Result<u16> {
    evaluate(circuit, "a", &mut HashMap::new()).ok_or(anyhow!("Failed to solve circuit for a"))
}

fn part2_solution(circuit: &HashMap<&str, &str>, part1_sol: u16) -> Result<u16> {
    let mut memo = HashMap::new();
    memo.insert("b", part1_sol);
    evaluate(circuit, "a", &mut memo).ok_or(anyhow!("Failed to solve circuit for a"))
}

fn evaluate<'a>(
    circuit: &HashMap<&'a str, &'a str>,
    desired_value: &'a str,
    memo: &mut HashMap<&'a str, u16>,
) -> Option<u16> {
    // if it's stored
    if let Some(val) = memo.get(desired_value) {
        return Some(*val);
    }
    // if it's just a raw vale
    if let Ok(val) = desired_value.parse::<u16>() {
        return Some(val);
    }

    // otherwise, parse it
    let expr = circuit.get(desired_value)?;

    let result = if let Ok(val) = expr.parse::<u16>() {
        Some(val)
    } else if !expr.contains(" ") {
        evaluate(circuit, expr, memo)
    } else if expr.contains("AND") {
        let (a, and, b) = expr.split_whitespace().collect_tuple::<(_, _, _)>()?;
        assert_eq!(and, "AND");
        Some(evaluate(circuit, a, memo)? & evaluate(circuit, b, memo)?)
    } else if expr.contains("OR") {
        let (a, or, b) = expr.split_whitespace().collect_tuple::<(_, _, _)>()?;
        assert_eq!(or, "OR");
        Some(evaluate(circuit, a, memo)? | evaluate(circuit, b, memo)?)
    } else if expr.contains("LSHIFT") {
        let (a, lshift, shift) = expr.split_whitespace().collect_tuple::<(_, _, _)>()?;
        assert_eq!(lshift, "LSHIFT");
        Some(evaluate(circuit, a, memo)? << evaluate(circuit, shift, memo)?)
    } else if expr.contains("RSHIFT") {
        let (a, rshift, shift) = expr.split_whitespace().collect_tuple::<(_, _, _)>()?;
        assert_eq!(rshift, "RSHIFT");
        Some(evaluate(circuit, a, memo)? >> evaluate(circuit, shift, memo)?)
    } else if expr.contains("NOT") {
        let (not, a) = expr.split_whitespace().collect_tuple::<(_, _)>()?;
        assert_eq!(not, "NOT");
        Some(!evaluate(circuit, a, memo)?)
    } else {
        None
    };

    // and store it
    if let Some(val) = result {
        memo.insert(desired_value, val);
    }
    result
}
