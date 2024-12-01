pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let mut me = Perso { hp: 50, mana: 500 };
    let mut boss = parse_input(input);

    let mut mana_spent = 0;
    while turn(&me, &boss, &mana_spent) {}

    input.to_string()
}

pub struct SpeelAttribute {
    pub speel: Speel,
    pub cost: u32,
    pub damage: Option<u32>,
    pub armor: Option<u32>,
    pub heal: Option<u32>,
    pub last: Option<u32>,
    pub mana_recup: Option<u32>,
}

pub enum Speel {
    MagicMissile,
    Drain,
    Sheild,
    Poison,
    Recharge,
}

impl Speel {
    pub fn get_all() -> Vec<SpeelAttribute> {
        let mut speels = vec![];
        speels.push(SpeelAttribute {
            speel: Self::MagicMissile,
            cost: 53,
            damage: Some(4),
            heal: None,
            last: None,
            mana_recup: None,
            armor: None,
        });
        speels.push(SpeelAttribute {
            speel: Self::Drain,
            cost: 73,
            damage: Some(2),
            heal: Some(2),
            armor: None,
            last: None,
            mana_recup: None,
        });
        speels.push(SpeelAttribute {
            speel: Self::Sheild,
            cost: 113,
            damage: None,
            armor: Some(7),
            last: Some(6),
            heal: None,
            mana_recup: None,
        });
        speels.push(SpeelAttribute {
            speel: Self::Poison,
            cost: 173,
            damage: Some(3),
            heal: None,
            last: Some(6),
            armor: None,
            mana_recup: None,
        });
        speels.push(SpeelAttribute {
            speel: Self::Recharge,
            cost: 229,
            damage: None,
            armor: None,
            last: Some(5),
            heal: None,
            mana_recup: Some(101),
        });

        speels
    }
}

pub fn turn(me: &Perso, boss: &Perso, &mut mana_spent: u32) -> bool {}

pub fn parse_input(input: &str) -> Perso {
    let mut lines = input.lines();
    let hp = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let mana = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    Perso { hp, mana }
}

#[derive(Debug, Clone)]
pub struct Perso {
    pub hp: i32,
    pub mana: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();
        assert_eq!("", output);
    }
}
