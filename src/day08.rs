use crate::load_input;
use anyhow::Result;

crate::aoc!(08);
pub fn day08_main() -> Result<()> {
    let input = load_input(8)?;

    assert_eq!(1342, part1_solution(&input));
    assert_eq!(2074, part2_solution(&input));

    Ok(())
}

fn part1_solution(input: &str) -> usize {
    let literal_chars: usize = input.lines().map(|s| s.len()).sum();
    let actual_chars: usize = input
        .lines()
        .map(|s| {
            let mut actual_len = s.len() - 2;
            let mut idx = 1;
            let bytes = s.as_bytes();
            while let Some(c) = bytes.get(idx) {
                match c {
                    b'\\' if bytes[idx + 1] == b'\\' || bytes[idx + 1] == b'\"' => {
                        actual_len -= 1;
                        idx += 2;
                        continue;
                    }
                    b'\\' if bytes[idx + 1] == b'x' => {
                        actual_len -= 3;
                        idx += 4;
                        continue;
                    }
                    _ => idx += 1,
                }
            }
            actual_len
        })
        .sum();

    literal_chars - actual_chars
}

fn part2_solution(input: &str) -> usize {
    let literal_chars: usize = input.lines().map(|s| s.len()).sum();
    let encoded_chars: usize = input
        .lines()
        .map(|s| {
            let mut encoded_len = 2; // start with 2 for the quotes
            for c in s.bytes() {
                match c {
                    b'\"' | b'\\' => encoded_len += 2,
                    _ => encoded_len += 1,
                }
            }
            encoded_len
        })
        .sum();

    encoded_chars - literal_chars
}
