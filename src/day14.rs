use crate::load_input;
use anyhow::Result;
use regex::Regex;

crate::aoc!(14);

pub fn day14_main() -> Result<()> {
    let input = load_input(14)?;
    let re = Regex::new(
        r"(?P<name>\w+) can fly (?P<speed>\d+) km/s for (?P<fly_time>\d+) seconds, but then must rest for (?P<rest_time>\d+) seconds.",
    )?;

    let mut reindeer = Vec::new();

    for caps in re.captures_iter(&input) {
        let stats = ReindeerStats {
            _name: caps["name"].to_owned(),
            speed: caps["speed"].parse::<u32>()?,
            fly_time: caps["fly_time"].parse::<u32>()?,
            rest_time: caps["rest_time"].parse::<u32>()?,
        };
        reindeer.push(stats);
    }

    assert_eq!(2660, part1_solution(&reindeer));
    assert_eq!(1256, part2_solution(&reindeer, 2503));

    Ok(())
}

struct ReindeerStats {
    _name: String,
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

fn part1_solution(reindeer: &[ReindeerStats]) -> u32 {
    reindeer
        .iter()
        .map(|stats| {
            let mut remaining_time = 2503 as i32;
            let mut distance = 0;
            loop {
                // first fly for fly_time
                if (stats.fly_time as i32) < remaining_time {
                    distance += stats.speed * stats.fly_time;
                    remaining_time -= stats.fly_time as i32;
                    // now rest
                    remaining_time -= stats.rest_time as i32;
                } else {
                    // time is up
                    // if remaining_time < 0, time ended while resting
                    // if remaining_time > 0, then it flew for the remainder of the time
                    if remaining_time > 0 {
                        // we're gonna
                        distance += stats.speed * remaining_time as u32;
                    }
                    break;
                }
            }
            distance
        })
        .max()
        .unwrap()
}

#[derive(Debug, Clone)]
enum Action {
    Flying(u32),
    Resting(u32),
}

#[derive(Debug, Clone)]
struct ReindeerPosition {
    distance: u32,
    points: u32,
    action: Action,
}

fn part2_solution(reindeer: &[ReindeerStats], race_time: u32) -> u32 {
    let mut positions = vec![
        ReindeerPosition {
            distance: 0,
            points: 0,
            action: Action::Flying(0),
        };
        reindeer.len()
    ];
    for _time in 1..=race_time {
        let mut first_reindeer_idx = reindeer.len() + 1;
        let mut first_reindeer_dist = 0;
        for (idx, (stats, pos)) in reindeer.iter().zip(positions.iter_mut()).enumerate() {
            match pos.action {
                Action::Flying(ref mut time) => {
                    if *time >= stats.fly_time {
                        pos.action = Action::Resting(1);
                    } else {
                        pos.distance += stats.speed;
                        *time += 1;
                    }
                }
                Action::Resting(ref mut time) => {
                    if *time >= stats.rest_time {
                        pos.action = Action::Flying(1);
                        pos.distance += stats.speed;
                    } else {
                        *time += 1;
                    }
                }
            }
            if pos.distance >= first_reindeer_dist {
                first_reindeer_idx = idx;
                first_reindeer_dist = pos.distance;
            }
        }
        positions[first_reindeer_idx].points += 1;
    }
    positions.iter().map(|x| x.points).max().unwrap()
}

#[test]
fn day14_examples() {
    println!(
        "{:?}",
        part2_solution(
            &[
                ReindeerStats {
                    _name: "Comet".to_owned(),
                    speed: 14,
                    fly_time: 10,
                    rest_time: 127,
                },
                ReindeerStats {
                    _name: "Dasher".to_owned(),
                    speed: 16,
                    fly_time: 11,
                    rest_time: 162,
                },
            ],
            1000
        )
    );
}
