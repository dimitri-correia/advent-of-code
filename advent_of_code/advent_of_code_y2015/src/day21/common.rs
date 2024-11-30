#[derive(Debug, Clone)]
pub struct Perso {
    pub hp: i32,
    pub damage: i32,
    pub armor: i32,
}

impl Perso {
    pub fn will_win(&self, other: &Perso) -> bool {
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

    pub fn parse_boss(input: &str) -> Self {
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
pub struct Shop {
    pub weapons: Vec<ShopObj>,
    pub armors: Vec<Option<ShopObj>>,
    pub rings: Vec<Option<ShopObj>>,
}

impl Shop {
    pub fn parse() -> Self {
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
pub struct ShopObj {
    pub cost: i32,
    pub damage: i32,
    pub armor: i32,
}
