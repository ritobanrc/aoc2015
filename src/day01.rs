use anyhow::{anyhow, Result};

#[test]
pub fn day01_main() -> Result<()> {
    use crate::load_input;
    let mut input = load_input(1)?;
    input.pop(); // remove newline

    assert_eq!(part1_solution(&input)?, 74);
    assert_eq!(part2_solution(&input)?, 1795);

    Ok(())
}

fn part1_solution(input: &str) -> Result<i64> {
    let mut count = 0;
    for c in input.chars() {
        match c {
            '(' => {
                count += 1;
            }
            ')' => {
                count -= 1;
            }
            _ => return Err(anyhow!("Unexpected character: {}", c)),
        }
    }
    Ok(count)
}

fn part2_solution(input: &str) -> Result<usize> {
    let mut count = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => {
                count += 1;
            }
            ')' => {
                count -= 1;
            }
            _ => return Err(anyhow!("Unexpected character: {}", c)),
        }

        if count < 0 {
            return Ok(i + 1); // answer is 1 indexed
        }
    }
    Err(anyhow!("Did not enter basement"))
}
