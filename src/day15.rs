use crate::load_input;
use anyhow::Result;
use regex::Regex;

crate::aoc!(15);

pub fn day15_main() -> Result<()> {
    let input = load_input(15)?;
    let re = Regex::new(
        r"(?P<name>\w+): capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>\d+)",
    )?;

    let mut ingedients = Vec::new();

    for caps in re.captures_iter(&input) {
        ingedients.push(Ingredient {
            name: caps["name"].to_owned(),
            capacity: caps["capacity"].parse::<i32>()?,
            durability: caps["durability"].parse::<i32>()?,
            flavor: caps["flavor"].parse::<i32>()?,
            texture: caps["texture"].parse::<i32>()?,
            calories: caps["calories"].parse::<u32>()?,
        });
    }

    assert_eq!(21367368, part1_solution(&ingedients));
    assert_eq!(1766400, part2_solution(&ingedients));

    Ok(())
}

fn calc_score1(stats: &[Ingredient], amounts: &[u32]) -> u32 {
    let sum_prop = |f: fn(&Ingredient) -> i32| {
        stats
            .iter()
            .zip(amounts)
            .map(|(stats, amount)| f(stats) * *amount as i32)
            .sum::<i32>()
            .clamp_gt(0)
    };

    let capacity = sum_prop(|s| s.capacity);
    let durability = sum_prop(|s| s.durability);
    let flavor = sum_prop(|s| s.flavor);
    let texture = sum_prop(|s| s.texture);
    (capacity * durability * flavor * texture) as u32
}

fn part1_solution(stats: &[Ingredient]) -> u32 {
    let mut best_score = 0;
    for i1 in 0..=100 {
        for i2 in 0..=100 - i1 {
            for i3 in 0..=100 - i1 - i2 {
                let i4 = 100 - i1 - i2 - i3;
                let score = calc_score1(stats, &[i1, i2, i3, i4]);
                if score > best_score {
                    best_score = score;
                }
            }
        }
    }
    best_score
}

fn calc_score2(stats: &[Ingredient], amounts: &[u32]) -> u32 {
    let sum_prop = |f: fn(&Ingredient) -> i32| {
        stats
            .iter()
            .zip(amounts)
            .map(|(stats, amount)| f(stats) * *amount as i32)
            .sum::<i32>()
            .clamp_gt(0)
    };

    if sum_prop(|s| s.calories as i32) != 500 {
        return 0;
    }

    let capacity = sum_prop(|s| s.capacity);
    let durability = sum_prop(|s| s.durability);
    let flavor = sum_prop(|s| s.flavor);
    let texture = sum_prop(|s| s.texture);
    (capacity * durability * flavor * texture) as u32
}

fn part2_solution(stats: &[Ingredient]) -> u32 {
    let mut best_score = 0;
    for i1 in 0..=100 {
        for i2 in 0..=100 - i1 {
            for i3 in 0..=100 - i1 - i2 {
                let i4 = 100 - i1 - i2 - i3;
                let score = calc_score2(stats, &[i1, i2, i3, i4]);
                if score > best_score {
                    best_score = score;
                }
            }
        }
    }
    best_score
}

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: u32,
}

trait ClampGreater: Ord {
    fn clamp_gt(self, min: Self) -> Self
    where
        Self: Sized;
}

impl<T> ClampGreater for T
where
    T: Ord,
{
    fn clamp_gt(self, min: Self) -> Self
    where
        Self: Sized,
    {
        if self < min {
            min
        } else {
            self
        }
    }
}

#[test]
fn day15_examples() {
    assert_eq!(
        62842880,
        part1_solution(&[
            Ingredient {
                name: "Butterscotch".to_owned(),
                capacity: -1,
                durability: -2,
                flavor: 6,
                texture: 3,
                calories: 8,
            },
            Ingredient {
                name: "Cinnamon".to_owned(),
                capacity: 2,
                durability: 3,
                flavor: -2,
                texture: -1,
                calories: 3
            }
        ])
    );
}
