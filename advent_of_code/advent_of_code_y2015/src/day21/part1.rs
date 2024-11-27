use core::str;
use std::{cmp::min, vec};

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let shop = Shop::parse();
    let boss = Perso::parse_boss(input);
    let me = Perso {
        hp: 100,
        damage: 0,
        armor: 0,
    };

    let mut min_cost = i32::MAX;
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
                    if me.will_win(&boss) {
                        min_cost = min(cost, min_cost);
                    }
                }
            }
        }
    }

    min_cost.to_string()
}

#[derive(Debug, Clone)]
struct Perso {
    hp: i32,
    damage: i32,
    armor: i32,
}

impl Perso {
    fn will_win(&self, other: &Perso) -> bool {
        let mut me = self.clone();
        let mut other = other.clone();
        loop {
            other.hp -= (me.damage - other.armor).max(1);
            if other.hp <= 0 {
                return true;
            }
            me.hp -= (other.damage - me.armor).max(1);
            if me.hp <= 0 {
                return false;
            }
        }
    }

    fn parse_boss(input: &str) -> Self {
        let mut hp = 0;
        let mut damage = 0;
        let mut armor = 0;
        input.lines().for_each(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 0 {
                return;
            }
            match parts[0] {
                "Hit" => hp = parts[2].parse().unwrap(),
                "Damage:" => damage = parts[1].parse().unwrap(),
                "Armor:" => armor = parts[1].parse().unwrap(),
                _ => panic!("Unknown type: {}", parts[0]),
            };
        });
        Perso { hp, damage, armor }
    }
}

#[derive(Debug)]
struct Shop {
    weapons: Vec<ShopObj>,
    armors: Vec<Option<ShopObj>>,
    rings: Vec<Option<ShopObj>>,
}

impl Shop {
    fn parse() -> Self {
        let input = include_str!("shop.txt").trim();
        let mut weapons = vec![];
        let mut armors = vec![None];
        let mut rings = vec![None];
        let mut actual = "";
        input.lines().for_each(|line| {
            let mut parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 0 {
                return;
            }
            let name = parts[0];
            if name == "Weapons:" || name == "Armor:" || name == "Rings:" {
                actual = name;
                return;
            }
            if parts.len() != 4 {
                parts = vec![parts[0], parts[2], parts[3], parts[4]];
            }
            let cost = parts[1].parse().unwrap();
            let damage = parts[2].parse().unwrap();
            let armor = parts[3].parse().unwrap();
            let obj = ShopObj {
                cost,
                damage,
                armor,
            };
            match actual {
                "Weapons:" => weapons.push(obj),
                "Armor:" => armors.push(Some(obj)),
                "Rings:" => rings.push(Some(obj)),
                _ => panic!("Unknown type: {}", actual),
            };
        });
        Shop {
            weapons,
            armors,
            rings,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ShopObj {
    cost: i32,
    damage: i32,
    armor: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();
        assert_eq!("111", output);
    }
}
