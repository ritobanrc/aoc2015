use crate::load_input;
use anyhow::Result;
use itertools::Itertools;

crate::aoc!(17);

pub fn day17_main() -> Result<()> {
    let input = load_input(17)?;
    let containers = input
        .lines()
        .map(|x| x.parse::<u32>())
        .collect::<Result<Vec<u32>, std::num::ParseIntError>>()?;

    assert_eq!(4372, part1_solution(&containers));
    assert_eq!(4, part2_solution(&containers));
    Ok(())
}

fn part1_solution(containers: &[u32]) -> usize {
    let mut num_combinations = 0;
    for num in 0..containers.len() {
        num_combinations += containers
            .iter()
            .combinations(num)
            .map(|c| c.into_iter().sum::<u32>())
            .filter(|sum| *sum == 150)
            .count();
    }
    num_combinations
}

fn part2_solution(containers: &[u32]) -> usize {
    for num in 0..containers.len() {
        let num_ways = containers
            .iter()
            .combinations(num)
            .map(|c| c.into_iter().sum::<u32>())
            .filter(|sum| *sum == 150)
            .count();
        if num_ways > 0 {
            return num_ways;
        }
    }
    0
}
