use anyhow::Result;
use itertools::Itertools;

crate::aoc!(11);

pub fn day11_main() -> Result<()> {
    const INPUT: &[u8] = b"cqjxjnds";

    let p1 = part1_solution(INPUT)?;
    assert_eq!("cqjxxyzz", &p1);
    assert_eq!("cqkaabcc", &part2_solution(&p1)?); // part 2 is just running part 1 over again

    Ok(())
}

fn part1_solution(input: &[u8]) -> Result<String> {
    let mut current = input.to_vec();
    while check_passwd(&current) == false {
        current = inc_string(&current);
    }
    Ok(String::from_utf8(current)?)
}

fn part2_solution(p1: &str) -> Result<String> {
    Ok(part1_solution(&inc_string(p1.as_bytes()))?)
}

fn check_passwd(input: &[u8]) -> bool {
    let three_inc = input
        .iter()
        .tuple_windows::<(_, _, _)>()
        .any(|(&a, &b, &c)| a + 1 == b && b + 1 == c);

    let iol = !(input.contains(&b'i') || input.contains(&b'l') || input.contains(&b'o'));

    let pair = input
        .iter()
        .tuple_windows::<(_, _)>()
        .filter(|(a, b)| a == b)
        .unique()
        .count()
        >= 2;

    three_inc && iol && pair
}

fn inc_string(input: &[u8]) -> Vec<u8> {
    fn inc_char(c: u8) -> u8 {
        if c == b'z' {
            b'a'
        } else {
            c + 1
        }
    }
    let right_char = inc_char(input[input.len() - 1]);
    let mut left = if right_char == b'a' {
        inc_string(&input[..input.len() - 1])
    } else {
        input[..input.len() - 1].to_vec()
    };
    left.push(right_char);
    left
}

#[test]
fn day11_examples() {
    assert_eq!(check_passwd(b"hijklmmn"), false);
    assert_eq!(check_passwd(b"abbceffg"), false);
    assert_eq!(check_passwd(b"ghjaabcc"), true);
    assert_eq!(check_passwd(b"abcdffaa"), true);

    assert_eq!(inc_string(b"xx"), b"xy");
    assert_eq!(inc_string(b"xy"), b"xz");
    assert_eq!(inc_string(b"xz"), b"ya");
    assert_eq!(inc_string(b"ya"), b"yb");
}
