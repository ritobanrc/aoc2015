use crate::{load_input, Grid2D};
use anyhow::{anyhow, Result};
use cgmath::{vec2, Vector2};
use itertools::iproduct;
use regex::Regex;

crate::aoc!(06);
pub fn day06_main() -> Result<()> {
    let input = load_input(6)?;
    let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)")?;
    let mut instructions = Vec::new();
    for line in input.lines() {
        let captures = re.captures(line).ok_or(anyhow!("Capture failed"))?;
        let op = match &captures[1] {
            "turn on" => Operation::TurnOn,
            "turn off" => Operation::TurnOff,
            "toggle" => Operation::Toggle,
            c => return Err(anyhow!("Unknown character sequence: {:?}", c)),
        };
        let min = vec2(captures[2].parse::<usize>()?, captures[3].parse::<usize>()?);
        let max = vec2(captures[4].parse::<usize>()?, captures[5].parse::<usize>()?);
        instructions.push(Instruction { op, min, max });
    }

    assert_eq!(569999, part1_solution(&instructions));
    assert_eq!(17836115, part2_solution(&instructions));

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    op: Operation,
    min: Vector2<usize>,
    max: Vector2<usize>,
}

fn part1_solution(instructions: &[Instruction]) -> usize {
    let mut grid = Grid2D::new(1000, 1000);

    for ins in instructions {
        for coord in iproduct!(ins.min.x..=ins.max.x, ins.min.y..=ins.max.y) {
            //println!("{:?}", coord);
            match ins.op {
                Operation::TurnOn => grid[coord] = true,
                Operation::TurnOff => grid[coord] = false,
                Operation::Toggle => grid[coord] = !grid[coord],
            }
        }
    }

    grid.iter()
        .filter(|(_x, _y, &filled)| filled == true)
        .count()
}

fn part2_solution(instructions: &[Instruction]) -> u32 {
    let mut grid: Grid2D<u32> = Grid2D::new(1000, 1000);

    for ins in instructions {
        for coord in iproduct!(ins.min.x..=ins.max.x, ins.min.y..=ins.max.y) {
            match ins.op {
                Operation::TurnOn => grid[coord] += 1,
                Operation::TurnOff if grid[coord] > 0 => grid[coord] -= 1,
                Operation::TurnOff => continue,
                Operation::Toggle => grid[coord] += 2,
            }
        }
    }

    grid.data.iter().sum()
}

#[test]
fn day06_examples() {
    assert_eq!(
        1,
        part2_solution(&[Instruction {
            op: Operation::TurnOn,
            min: vec2(0, 0),
            max: vec2(0, 0),
        }])
    );

    assert_eq!(
        2000000,
        part2_solution(&[Instruction {
            op: Operation::Toggle,
            min: vec2(0, 0),
            max: vec2(999, 999),
        }])
    );
}
