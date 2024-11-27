pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    let nb_present_to_reach = input.trim().parse::<u32>().unwrap();

    let highest_house_number = nb_present_to_reach / 11; // as each elf delivers at least 11 presents to each house
    let mut presents = vec![0; highest_house_number as usize];

    (1..highest_house_number).into_iter().for_each(|elf| {
        let mut house_number = elf;
        let mut nb_houses = elf;
        while house_number < highest_house_number && nb_houses > 0 {
            presents[house_number as usize] += elf * 10;
            house_number += elf;
            nb_houses -= 1;
        }
    });

    presents
        .into_iter()
        .position(|p| p >= nb_present_to_reach)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();
        assert_eq!("831600", output);
    }
}
