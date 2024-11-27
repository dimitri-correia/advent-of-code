pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let nb_present_to_reach = input.trim().parse::<u32>().unwrap();

    let highest_house_number = nb_present_to_reach / 10; // as each elf delivers at least 10 presents to each house
    let mut presents = vec![0; highest_house_number as usize];

    (1..highest_house_number).into_iter().for_each(|elf| {
        let mut house_number = elf;
        while house_number < highest_house_number {
            presents[house_number as usize] += elf * 10;
            house_number += elf;
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
        let output = part_1_actual_challenge();
        assert_eq!("786240", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("8", r);
    }
}
