use anyhow::Result;
use itertools::iproduct;
use itertools::Itertools;

crate::aoc!(21);

pub fn day21_main() -> Result<()> {
    let weapons = vec![
        Stats {
            name: "Dagger".to_owned(),
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Stats {
            name: "Shortsword".to_owned(),
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Stats {
            name: "Warhammer".to_owned(),
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Stats {
            name: "Longsword".to_owned(),
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Stats {
            name: "Greataxe".to_owned(),
            cost: 74,
            damage: 8,
            armor: 0,
        },
    ];

    let armor = vec![
        Stats {
            name: "None".to_owned(),
            cost: 0,
            damage: 0,
            armor: 0,
        },
        Stats {
            name: "Leather".to_owned(),
            cost: 13,
            damage: 0,
            armor: 1,
        },
        Stats {
            name: "Chainmail".to_owned(),
            cost: 31,
            damage: 0,
            armor: 2,
        },
        Stats {
            name: "Splintmail".to_owned(),
            cost: 53,
            damage: 0,
            armor: 3,
        },
        Stats {
            name: "Bandedmail".to_owned(),
            cost: 75,
            damage: 0,
            armor: 4,
        },
        Stats {
            name: "Platemail".to_owned(),
            cost: 102,
            damage: 0,
            armor: 5,
        },
    ];

    let rings = vec![
        Stats {
            name: "None".to_owned(),
            cost: 0,
            damage: 0,
            armor: 0,
        },
        Stats {
            name: "Damage +1 ".to_owned(),
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Stats {
            name: "Damage +2 ".to_owned(),
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Stats {
            name: "Damage +3 ".to_owned(),
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Stats {
            name: "Defense +1".to_owned(),
            cost: 20,
            damage: 0,
            armor: 1,
        },
        Stats {
            name: "Defense +2".to_owned(),
            cost: 40,
            damage: 0,
            armor: 2,
        },
        Stats {
            name: "Defense +3".to_owned(),
            cost: 80,
            damage: 0,
            armor: 3,
        },
    ];
    let enemy = Player {
        hp: 103,
        damage: 9,
        armor: 2,
    };
    assert_eq!(121, part1_solution(&weapons, &armor, &rings, &enemy, 100));
    assert_eq!(201, part2_solution(&weapons, &armor, &rings, &enemy, 100));
    Ok(())
}

#[derive(Debug)]
struct Stats {
    name: String,
    cost: u32,
    damage: i32,
    armor: i32,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub hp: i32,
    pub damage: i32,
    pub armor: i32,
}

fn part1_solution(
    weapons: &[Stats],
    armors: &[Stats],
    rings: &[Stats],
    enemy: &Player,
    hp: i32,
) -> u32 {
    let mut best_cost = u32::MAX;
    // Note, this solution does not handle the possibility of using 0 rings.
    for possibility in iproduct!(weapons, armors, rings.iter().combinations(2)) {
        let cost =
            possibility.0.cost + possibility.1.cost + possibility.2[0].cost + possibility.2[1].cost;
        if cost > best_cost {
            continue; // no point in checking this one
        }
        let mut enemy = enemy.to_owned();
        let mut player = Player {
            hp,
            damage: possibility.0.damage
                + possibility.1.damage
                + possibility.2[0].damage
                + possibility.2[1].damage,
            armor: possibility.0.armor
                + possibility.1.armor
                + possibility.2[0].armor
                + possibility.2[1].armor,
        };
        let mut damager = &mut player;
        let mut damagee = &mut enemy;
        while damager.hp > 0 && damagee.hp > 0 {
            damagee.hp -= (damager.damage - damagee.armor).max(1);
            // at the end of each turn, swap damager and damagee
            std::mem::swap(&mut damager, &mut damagee);
        }
        // if the player survived this encounter
        if player.hp > 0 {
            best_cost = cost;
        }
    }
    best_cost
}

fn part2_solution(
    weapons: &[Stats],
    armors: &[Stats],
    rings: &[Stats],
    enemy: &Player,
    hp: i32,
) -> u32 {
    let mut worst_cost = 0;
    // Note, this solution does not handle the possibility of using 0 rings.
    for possibility in iproduct!(weapons, armors, rings.iter().combinations(2)) {
        let cost =
            possibility.0.cost + possibility.1.cost + possibility.2[0].cost + possibility.2[1].cost;
        if cost < worst_cost {
            continue; // no point in checking this one
        }
        let mut enemy = enemy.to_owned();
        let mut player = Player {
            hp,
            damage: possibility.0.damage
                + possibility.1.damage
                + possibility.2[0].damage
                + possibility.2[1].damage,
            armor: possibility.0.armor
                + possibility.1.armor
                + possibility.2[0].armor
                + possibility.2[1].armor,
        };
        let mut damager = &mut player;
        let mut damagee = &mut enemy;
        while damager.hp > 0 && damagee.hp > 0 {
            damagee.hp -= (damager.damage - damagee.armor).max(1);
            // at the end of each turn, swap damager and damagee
            std::mem::swap(&mut damager, &mut damagee);
        }
        // if the player lost this encounter
        if enemy.hp > 0 {
            worst_cost = cost;
        }
    }
    worst_cost
}
