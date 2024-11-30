use crate::day16::common::Sue;

pub fn part_2_actual_challenge() -> String {
    let input_memories = include_str!("input1.txt");
    let input_actual_sue = include_str!("input2.txt");
    part_2(&input_memories, &input_actual_sue)
}

fn part_2(input_memories: &str, input_actual_sue: &str) -> String {
    let memories = Sue::parse_memories(input_memories);
    let actual_sue = Sue::parse_actual_sue(input_actual_sue);
    let filtered: Vec<&Sue> = memories
        .iter()
        .filter(|memory_sue| {
            !(memory_sue.children.is_some() && memory_sue.children != actual_sue.children)
                && !(memory_sue.cats.is_some() && memory_sue.cats <= actual_sue.cats)
                && !(memory_sue.samoyeds.is_some() && memory_sue.samoyeds != actual_sue.samoyeds)
                && !(memory_sue.pomeranians.is_some()
                    && memory_sue.pomeranians >= actual_sue.pomeranians)
                && !(memory_sue.akitas.is_some() && memory_sue.akitas != actual_sue.akitas)
                && !(memory_sue.vizslas.is_some() && memory_sue.vizslas != actual_sue.vizslas)
                && !(memory_sue.goldfish.is_some() && memory_sue.goldfish >= actual_sue.goldfish)
                && !(memory_sue.trees.is_some() && memory_sue.trees <= actual_sue.trees)
                && !(memory_sue.cars.is_some() && memory_sue.cars != actual_sue.cars)
                && !(memory_sue.perfumes.is_some() && memory_sue.perfumes != actual_sue.perfumes)
        })
        .collect();

    assert!(filtered.len() == 1);
    filtered[0].number.unwrap().to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();
        assert_eq!("405", output);
    }
}
