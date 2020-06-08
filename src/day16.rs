use crate::load_input;
use anyhow::Result;
use itertools::Itertools;

crate::aoc!(16);

pub fn day16_main() -> Result<()> {
    let input = load_input(16)?;

    let mut sues = Vec::new();

    for (i, line) in input.lines().enumerate() {
        sues.push([None; 10]);
        for data in line[line.find(':').unwrap() + 2..].split(',') {
            let (name, count) = data.splitn(2, ": ").collect_tuple::<(_, _)>().unwrap();
            let name = name.trim();
            let count = count.parse::<u32>()?;
            sues[i][idx(name)] = Some(count);
        }
    }

    assert_eq!(40, part1_solution(&sues));
    assert_eq!(241, part2_solution(&sues));

    Ok(())
}

fn part2_solution(sues: &[[Option<u32>; 10]]) -> usize {
    /// children: 0
    /// cats: 1
    /// samoyeds: 2
    /// pomeranians: 3
    /// akitas: 4
    /// vizslas: 5
    /// goldfish: 6
    /// trees: 7
    /// cars: 8
    /// perfumes: 9
    const REPORTED: [u32; 10] = [3, 7, 2, 3, 0, 0, 5, 3, 2, 1];
    let mut possibilities = vec![true; sues.len()];
    for datum_idx in 0..10usize {
        for (sue, possible) in sues.iter().zip(&mut possibilities) {
            if !*possible {
                continue;
            }
            if let Some(datum) = sue[datum_idx] {
                match datum_idx {
                    // because the REPORTED for cats and trees are lower than the
                    // actual, if the REPORTED is greater than the actual for this Sue,
                    // it's impossible for it to have been this sue
                    1 | 7 if REPORTED[datum_idx] >= datum => *possible = false,
                    3 | 6 if REPORTED[datum_idx] <= datum => *possible = false,
                    2 | 4 | 5 | 8 | 9 if REPORTED[datum_idx] != datum => *possible = false,
                    _ => continue,
                }
            }
        }
    }
    possibilities
        .iter()
        .find_position(|&&x| x == true)
        .unwrap()
        .0
        + 1
}

fn part1_solution(sues: &[[Option<u32>; 10]]) -> usize {
    /// children: 3
    /// cats: 7
    /// samoyeds: 2
    /// pomeranians: 3
    /// akitas: 0
    /// vizslas: 0
    /// goldfish: 5
    /// trees: 3
    /// cars: 2
    /// perfumes: 1
    const ACTUAL: [u32; 10] = [3, 7, 2, 3, 0, 0, 5, 3, 2, 1];
    let mut possibilities = vec![true; sues.len()];
    for datum_idx in 0..10usize {
        for (sue, possible) in sues.iter().zip(&mut possibilities) {
            if !*possible {
                continue;
            }
            if let Some(datum) = sue[datum_idx] {
                if datum != ACTUAL[datum_idx] {
                    *possible = false;
                }
            }
        }
    }
    possibilities
        .iter()
        .find_position(|&&x| x == true)
        .unwrap()
        .0
        + 1
}

fn idx(s: &str) -> usize {
    match s {
        "children" => 0,
        "cats" => 1,
        "samoyeds" => 2,
        "pomeranians" => 3,
        "akitas" => 4,
        "vizslas" => 5,
        "goldfish" => 6,
        "trees" => 7,
        "cars" => 8,
        "perfumes" => 9,
        s => panic!("Unrecognized string: {:?}", s),
    }
}
