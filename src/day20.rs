use anyhow::Result;

crate::aoc!(20);

pub fn day20_main() -> Result<()> {
    const INPUT: u32 = 33100000;
    assert_eq!(776160, part1_solution(INPUT));
    assert_eq!(786240, part2_solution(INPUT));
    Ok(())
}

fn part1_solution(num_presents: u32) -> usize {
    // a trivial upper bound for this problem is num_presents / 10 houses, because each house
    // gets at least 10x it's house number presents (from the elf with the same number as the
    // house).
    let mut presents = vec![0u32; num_presents as usize / 10];
    for elf in 1..presents.len() {
        for n in 1.. {
            if n * elf >= presents.len() {
                break;
            }
            presents[n * elf - 1] += (10 * elf) as u32;
        }
    }
    presents
        .iter()
        .enumerate()
        .find(|(_house, &val)| val >= num_presents)
        .unwrap()
        .0
        + 1
}

fn part2_solution(num_presents: u32) -> usize {
    // a trivial upper bound for this problem is num_presents / 10 houses, because each house
    // gets at least 10x it's house number presents (from the elf with the same number as the
    // house).
    let mut presents = vec![0u32; num_presents as usize / 11];
    for elf in 1..presents.len() {
        for n in 1..=50 {
            if n * elf >= presents.len() {
                break;
            }
            presents[n * elf - 1] += (11 * elf) as u32;
        }
    }
    presents
        .iter()
        .enumerate()
        .find(|(_house, &val)| val >= num_presents)
        .unwrap()
        .0
        + 1
}
