use std::cmp::max;

use super::common::{Perso, Shop};

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    let shop = Shop::parse();
    let boss = Perso::parse_boss(input);
    let me = Perso {
        hp: 100,
        damage: 0,
        armor: 0,
    };

    let mut max_cost = 0;
    for weapon in &shop.weapons {
        // weapon 1
        for armor in &shop.armors {
            // armor 0 or 1
            for ring1 in &shop.rings {
                for ring2 in &shop.rings {
                    if ring1 == ring2 && ring1.is_some() {
                        // ring 0, 1, or 2
                        continue;
                    }
                    let cost = weapon.cost
                        + armor.as_ref().map_or(0, |a| a.cost)
                        + ring1.as_ref().map_or(0, |r| r.cost)
                        + ring2.as_ref().map_or(0, |r| r.cost);
                    let damage = weapon.damage
                        + armor.as_ref().map_or(0, |a| a.damage)
                        + ring1.as_ref().map_or(0, |r| r.damage)
                        + ring2.as_ref().map_or(0, |r| r.damage);
                    let armor = weapon.armor
                        + armor.as_ref().map_or(0, |a| a.armor)
                        + ring1.as_ref().map_or(0, |r| r.armor)
                        + ring2.as_ref().map_or(0, |r| r.armor);

                    let mut me = me.clone();
                    me.damage = damage;
                    me.armor = armor;
                    if boss.will_win(&me) {
                        max_cost = max(cost, max_cost);
                    }
                }
            }
        }
    }

    max_cost.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();
        assert_eq!("188", output);
    }
}
