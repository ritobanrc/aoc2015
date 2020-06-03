use anyhow::{anyhow, Result};
use cgmath::vec2;
use std::collections::HashSet;

#[test]
pub fn day03_main() -> Result<()> {
    use crate::load_input;
    let mut input = load_input(3)?;
    input.pop(); // remove newline

    assert_eq!(2592, part1_solution(&input)?);

    assert_eq!(2360, part2_solution(&input)?);

    Ok(())
}

fn part1_solution(input: &str) -> Result<usize> {
    let mut pos = vec2(0, 0);
    let mut map = HashSet::new();
    map.insert(pos);
    for dir in input.chars() {
        match dir {
            '>' => pos += vec2(1, 0),
            '<' => pos += vec2(-1, 0),
            '^' => pos += vec2(0, 1),
            'v' => pos += vec2(0, -1),
            c => return Err(anyhow!("Error: Unexpected character: {}", c)),
        }
        map.insert(pos);
    }
    Ok(map.iter().count())
}

fn part2_solution(input: &str) -> Result<usize> {
    let mut santas = [vec2(0, 0); 2];
    let mut map = HashSet::new();
    map.insert(santas[0]);
    map.insert(santas[1]);
    for (idx, dir) in input.chars().enumerate() {
        let pos = &mut santas[idx % 2];
        match dir {
            '>' => *pos += vec2(1, 0),
            '<' => *pos += vec2(-1, 0),
            '^' => *pos += vec2(0, 1),
            'v' => *pos += vec2(0, -1),
            c => return Err(anyhow!("Error: Unexpected character: {}", c)),
        }
        map.insert(*pos);
    }
    Ok(map.iter().count())
}
