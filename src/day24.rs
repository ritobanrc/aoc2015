use crate::load_input;
use anyhow::Result;
use itertools::Itertools;

crate::aoc!(24);

pub fn day24_main() -> Result<()> {
    let input = load_input(24)?
        .lines()
        .map(str::parse::<u32>)
        .collect::<Result<Vec<_>, _>>()?;

    assert_eq!(11266889531, part1_solution(&input));
    assert_eq!(77387711, part2_solution(&input));
    Ok(())
}

fn part1_solution(weights: &[u32]) -> u64 {
    let bucket_sum = weights.iter().sum::<u32>() / 3;
    let mut min_quantum_entanglement = u64::MAX;

    for num_front_packages in 1..weights.len() {
        'front: for front_packages in weights.iter().combinations(num_front_packages) {
            if front_packages.iter().map(|x| **x).sum::<u32>() != bucket_sum {
                continue;
            }
            let remaining: Vec<_> = weights
                .iter()
                .filter(|x| !front_packages.contains(x))
                .collect();

            // so now find the ways you can split up the remaining
            for back_possibility in remaining.iter().map(|_| 0..2).multi_cartesian_product() {
                // back_possibility is a Vec which each element is either 0 or 1,
                // based on which location it was put into

                // check the sum of one of the back buckets
                let back1_weight = back_possibility
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &loc)| if loc == 0 { Some(remaining[i]) } else { None })
                    .sum::<u32>();

                if back1_weight != bucket_sum {
                    continue;
                } else {
                    // this is a valid solution, all 3 locations must have the same
                    // weight
                    let quantum_entanglement = front_packages.iter().map(|x| **x as u64).product();
                    if quantum_entanglement < min_quantum_entanglement {
                        min_quantum_entanglement = quantum_entanglement;
                        break 'front; // the first result also happens to be the best result, and it takes a really long time to go through all the other possibilities.
                    }
                }
            }
        }
        if min_quantum_entanglement != u64::MAX {
            break; // we prioritize minimizing num_front_packages, so no point in checking great values of num_front_packages
        }
    }
    min_quantum_entanglement
}

fn part2_solution(weights: &[u32]) -> u64 {
    let bucket_sum = weights.iter().sum::<u32>() / 4;
    let mut min_quantum_entanglement = u64::MAX;

    for num_front_packages in 1..weights.len() {
        'front: for front_packages in weights.iter().combinations(num_front_packages) {
            if front_packages.iter().map(|x| **x).sum::<u32>() != bucket_sum {
                continue;
            }
            let remaining: Vec<_> = weights
                .iter()
                .filter(|x| !front_packages.contains(x))
                .collect();

            // so now find the ways you can split up the remaining
            for back_possibility in remaining.iter().map(|_| 0..3).multi_cartesian_product() {
                // back_possibility is a Vec which each element is either 0 or 1,
                // based on which location it was put into

                // check the sum of two of the back buckets
                let back1_weight = back_possibility
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &loc)| if loc == 0 { Some(remaining[i]) } else { None })
                    .sum::<u32>();

                // check the sum of two of the back buckets
                let back2_weight = back_possibility
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &loc)| if loc == 1 { Some(remaining[i]) } else { None })
                    .sum::<u32>();

                if back1_weight != bucket_sum || back2_weight != bucket_sum {
                    continue;
                } else {
                    // this is a valid solution, all 3 locations must have the same
                    // weight
                    let quantum_entanglement = front_packages.iter().map(|x| **x as u64).product();
                    if quantum_entanglement < min_quantum_entanglement {
                        min_quantum_entanglement = quantum_entanglement;
                        // the first result also happens to be the best result, and it takes a really long time to go through all the other possibilities.
                        break 'front;
                    }
                }
            }
        }
        if min_quantum_entanglement != u64::MAX {
            break; // we prioritize minimizing num_front_packages, so no point in checking great values of num_front_packages
        }
    }
    min_quantum_entanglement
}
