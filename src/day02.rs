use anyhow::{anyhow, Result};
use itertools::Itertools;

#[test]
pub fn day02_main() -> Result<()> {
    use crate::load_input;
    let input: Vec<_> = load_input(2)?
        .lines()
        .flat_map(|l| l.split("x"))
        .map(|x| x.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()? // this just to terminate if any of the parses failed
        .into_iter()
        .tuples::<(_, _, _)>()
        .collect();

    assert_eq!(1588178, part1_solution(&input));
    assert_eq!(3783758, part2_solution(&input));
    Ok(())
}

fn part1_solution(dims: &[(u32, u32, u32)]) -> u32 {
    dims.iter()
        .map(|d| {
            let s1 = d.0 * d.1;
            let s2 = d.1 * d.2;
            let s3 = d.0 * d.2;
            let extra = s1.min(s2).min(s3);
            2 * (s1 + s2 + s3) + extra
        })
        .sum()
}

fn part2_solution(dims: &[(u32, u32, u32)]) -> u32 {
    dims.iter()
        .map::<u32, _>(|d| {
            let mut d = [d.0, d.1, d.2];

            let bow_size: u32 = d.iter().product();

            let min_idx = d.iter().position_min().unwrap();
            let min = d[min_idx];
            d[min_idx] = u32::MAX; // get rid of the largest value
            let second_min = d.iter().min().unwrap();

            let ribbon_length = 2 * min + 2 * second_min + bow_size;
            ribbon_length
        })
        .sum()
}

#[test]
fn day02_test() {
    assert_eq!(48, part2_solution(&[(2, 3, 4), (1, 1, 10)]));
}
