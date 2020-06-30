use crate::day21::Player;
use anyhow::Result;

crate::aoc!(22);

pub fn day22_main() -> Result<()> {
    let player = Player {
        hp: 50,
        damage: 0,
        armor: 0,
    };
    let boss = Player {
        hp: 51,
        damage: 9,
        armor: 0,
    };
    let spells = vec![
        Spell {
            spell_type: SpellType::MagicMissle,
            mana: 53,
        },
        Spell {
            spell_type: SpellType::Drain,
            mana: 73,
        },
        Spell {
            spell_type: SpellType::Shield,
            mana: 113,
        },
        Spell {
            spell_type: SpellType::Poison,
            mana: 173,
        },
        Spell {
            spell_type: SpellType::Recharge,
            mana: 229,
        },
    ];
    dbg!(part1_solution(
        &spells,
        500,
        &player,
        &boss,
        &vec![],
        i32::MAX
    ));
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct Spell {
    spell_type: SpellType,
    mana: i32,
}

#[derive(Debug, Clone, PartialEq)]
enum SpellType {
    MagicMissle,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug, Clone)]
struct Effect {
    spell_type: SpellType,
    turns_left: usize,
}

fn part1_solution(
    spells: &[Spell],
    mana: i32,
    player: &Player,
    enemy: &Player,
    effects: &[Effect],
    mut best_mana: i32,
) -> i32 {
    for next_spell in spells {
        if next_spell.mana + mana > best_mana {
            return best_mana; // this isn't a useful path to explore
        }
        let mut enemy = enemy.to_owned();
        let mut player = player.to_owned();
        let mut next_mana = mana;
        // if next_spell.mana > best_mana {
        // if just the next spell would cost more than a previously accepted pathway to winning,
        // don't bother checking it. also, since spells are listed in ascending order of mana,
        // you can skip all future spells too!
        // break;
        // }
        //println!("{:?}", next_spell);
        // Apply effects first
        let mut effects = effects
            .iter()
            .filter_map(|effect| {
                let mut effect = effect.to_owned();
                effect.turns_left -= 1;
                //println!("Applying effect: {:?}", effect);
                match effect.spell_type {
                    SpellType::MagicMissle | SpellType::Drain => unreachable!(),
                    SpellType::Shield => player.armor = 7,
                    SpellType::Poison => enemy.hp -= 3,
                    SpellType::Recharge => next_mana += 101,
                }
                if effect.turns_left > 0 {
                    Some(effect)
                } else {
                    None
                }
            })
            .collect::<Vec<Effect>>();

        // after the effects, check if the enemy was killed
        if enemy.hp <= 0 {
            // great, we win, having spent no mana this turn
            // this is the best pathway forward, so immediately return 0 mana
            //println!("We've won?");
            return 0;
        }

        // make sure that we don't re-cast an already existing effect!
        if effects
            .iter()
            .any(|e| e.spell_type == next_spell.spell_type)
        {
            //println!(
            //"Cannot cast {:?} because it already is an effect",
            //next_spell
            //);
            continue;
        }

        // try applying this spell + any effects this turn,
        // check if anyone has won,
        //    if so, return the total amount of mana used
        //    if not, recursively call this function
        if next_spell.mana <= next_mana {
            //println!("Casting spell: {:?}", next_spell);
            next_mana -= next_spell.mana;
            // cast this spill
            match next_spell.spell_type {
                SpellType::MagicMissle => enemy.hp -= 4,
                SpellType::Drain => {
                    enemy.hp -= 2;
                    player.hp += 2;
                }
                SpellType::Shield => effects.push(Effect {
                    spell_type: SpellType::Shield,
                    turns_left: 6,
                }),
                SpellType::Poison => effects.push(Effect {
                    spell_type: SpellType::Poison,
                    turns_left: 6,
                }),
                SpellType::Recharge => effects.push(Effect {
                    spell_type: SpellType::Recharge,
                    turns_left: 5,
                }),
            }
        } else {
            // On each of your turns, you **must** select one of your spells to cast
            // If we can't afford this, skip it
            //println!("Not enough mana to cast: {:?}", next_spell);
            continue;
        }

        // cast all of the effects again
        let effects = effects
            .iter()
            .filter_map(|effect| {
                let mut effect = effect.to_owned();
                effect.turns_left -= 1;
                //println!("Applying effect: {:?}", effect);
                match effect.spell_type {
                    SpellType::MagicMissle | SpellType::Drain => unreachable!(),
                    SpellType::Shield => player.armor = 7,
                    SpellType::Poison => enemy.hp -= 3,
                    SpellType::Recharge => next_mana += 101,
                }
                if effect.turns_left > 0 {
                    Some(effect)
                } else {
                    None
                }
            })
            .collect::<Vec<Effect>>();

        // after the turn + effects, check if the enemy was killed
        //println!("Enemy: {:?}", enemy);
        if enemy.hp <= 0 {
            // great, we win by spending next_spell.mana
            best_mana = best_mana.min(next_spell.mana);
            println!("We've won!");
            // continue to check for a more efficient way to win
            continue;
        }

        // Now, the Enemy's turn
        player.hp -= (enemy.damage - player.armor).max(1);

        //println!("Player: {:?}", player);
        //println!();

        if player.hp <= 0 {
            // if we lose, do nothing.
            continue;
        }

        // since its not over, go on to the next turn
        let this_mana = part1_solution(spells, next_mana, &player, &enemy, &effects, best_mana);
        if this_mana == i32::MAX {
            // this is not a viable pathway no matter what
            continue;
        }
        best_mana = best_mana.min(this_mana + next_spell.mana);

        //println!("Found solution using {:?} mana", this_mana);
    }
    best_mana
}
