use anyhow::{anyhow, Result};
use itertools::Itertools;

#[test]
pub fn day05_main() -> Result<()> {
    use crate::load_input;
    let input = load_input(5)?;

    assert_eq!(258, part1_solution(&input));
    assert_eq!(53, part2_solution(&input));

    Ok(())
}

fn is_nice_p1(data: &str) -> bool {
    // first, count vowels
    let vowels = data
        .chars()
        .filter(|x| match x {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        })
        .count();

    let dup = data.chars().tuple_windows::<(_, _)>().any(|(l, r)| l == r);

    let forbidden = data.chars().tuple_windows::<(_, _)>().any(|x| match x {
        ('a', 'b') => true,
        ('c', 'd') => true,
        ('p', 'q') => true,
        ('x', 'y') => true,
        _ => false,
    });

    vowels >= 3 && dup && !forbidden
}

fn part1_solution(input: &str) -> usize {
    input.lines().filter(|x| is_nice_p1(x)).count()
}

fn is_nice_p2(data: &str) -> bool {
    fn has_dup_pair(data: &str) -> bool {
        for (i, pair1) in data.chars().tuple_windows::<(_, _)>().enumerate() {
            for (j, pair2) in data.chars().tuple_windows::<(_, _)>().enumerate() {
                if pair1 == pair2 && (i as isize - j as isize).abs() >= 2 {
                    return true;
                }
            }
        }
        false
    }
    let dup_pair = has_dup_pair(data);

    let xyx = data
        .chars()
        .tuple_windows::<(_, _, _)>()
        .any(|(a, _, b)| a == b);

    dup_pair && xyx
}

fn part2_solution(input: &str) -> usize {
    input.lines().filter(|x| is_nice_p2(x)).count()
}
