use crate::load_input;
use anyhow::Result;
use regex::Regex;

crate::aoc!(12);

pub fn day12_main() -> Result<()> {
    let input = load_input(12)?;
    assert_eq!(111754, part1_solution(&input)?);
    assert_eq!(65402, part2_solution(&input)?);
    Ok(())
}

fn part1_solution(input: &str) -> Result<i64> {
    let re = Regex::new(r"(-?\d+)")?;
    Ok(re
        .captures_iter(input)
        .filter_map(|cap| cap[1].parse::<i64>().ok())
        .sum())
}

fn part2_solution(input: &str) -> Result<i64> {
    use json::JsonValue;
    let json = json::parse(input)?;
    fn sum_except_red(value: &JsonValue) -> i64 {
        match value {
            JsonValue::Object(obj) => {
                if obj
                    .iter()
                    .any(|(_key, value)| value.as_str().map(|x| x == "red").unwrap_or(false))
                {
                    // if for any key, value pair in this object,
                    0
                } else {
                    obj.iter().map(|(_key, value)| sum_except_red(value)).sum()
                }
            }
            JsonValue::Number(num) => num.as_fixed_point_i64(0).unwrap(),
            JsonValue::Array(vec) => vec.iter().map(sum_except_red).sum(),
            _ => 0,
        }
    }
    Ok(sum_except_red(&json))
}
