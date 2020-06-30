use crate::load_input;
use anyhow::{anyhow, Result};

crate::aoc!(23);
pub fn day23_main() -> Result<()> {
    let input = load_input(23)?;
    let lines: Vec<_> = input.lines().collect();

    assert_eq!(307, solutions(&lines, 0).ok_or(anyhow!("Part 1 failed."))?);
    assert_eq!(160, solutions(&lines, 1).ok_or(anyhow!("Part 2 failed."))?);
    Ok(())
}

fn solutions(lines: &[&str], init_a: u32) -> Option<u32> {
    let mut a = init_a;
    let mut b = 0;

    let mut current_line = 0;

    while current_line < lines.len() {
        let mut pieces = lines[current_line]
            .split(|c| c == ' ' || c == ',')
            .filter(|s| s != &"");

        match pieces.next()? {
            "hlf" => match pieces.next()? {
                "a" => a /= 2,
                "b" => b /= 2,
                c => panic!("Unrecognized char: {:?}", c),
            },
            "tpl" => match pieces.next()? {
                "a" => a *= 3,
                "b" => b *= 3,
                c => panic!("Unrecognized char: {:?}", c),
            },
            "inc" => match pieces.next()? {
                "a" => a += 1,
                "b" => b += 1,
                c => panic!("Unrecognized char: {:?}", c),
            },
            "jmp" => {
                current_line =
                    (current_line as isize + pieces.next()?.parse::<isize>().ok()?) as usize;
                continue;
            }
            "jie" => {
                let reg = match pieces.next()? {
                    "a" => a,
                    "b" => b,
                    c => panic!("Unrecognized char: {:?}", c),
                };
                if reg % 2 == 0 {
                    current_line =
                        (current_line as isize + pieces.next()?.parse::<isize>().ok()?) as usize;
                    continue;
                }
            }
            "jio" => {
                let reg = match pieces.next()? {
                    "a" => a,
                    "b" => b,
                    c => panic!("Unrecognized char: {:?}", c),
                };
                if reg == 1 {
                    current_line =
                        (current_line as isize + pieces.next()?.parse::<isize>().ok()?) as usize;
                    continue;
                }
            }
            _ => return None,
        }
        current_line += 1;
    }

    Some(b)
}
