pub fn part_1_actual_challenge() -> String {
    let input_memories = include_str!("input1.txt");
    let input_actual_sue = include_str!("input2.txt");
    part_1(&input_memories, &input_actual_sue)
}

fn part_1(input_memories: &str, input_actual_sue: &str) -> String {
    let memories = Sue::parse_memories(input_memories);
    let actual_sue = Sue::parse_actual_sue(input_actual_sue);
    let filtered: Vec<&Sue> = memories
        .iter()
        .filter(|memory_sue| {
            !(memory_sue.children.is_some() && memory_sue.children != actual_sue.children)
                && !(memory_sue.cats.is_some() && memory_sue.cats != actual_sue.cats)
                && !(memory_sue.samoyeds.is_some() && memory_sue.samoyeds != actual_sue.samoyeds)
                && !(memory_sue.pomeranians.is_some()
                    && memory_sue.pomeranians != actual_sue.pomeranians)
                && !(memory_sue.akitas.is_some() && memory_sue.akitas != actual_sue.akitas)
                && !(memory_sue.vizslas.is_some() && memory_sue.vizslas != actual_sue.vizslas)
                && !(memory_sue.goldfish.is_some() && memory_sue.goldfish != actual_sue.goldfish)
                && !(memory_sue.trees.is_some() && memory_sue.trees != actual_sue.trees)
                && !(memory_sue.cars.is_some() && memory_sue.cars != actual_sue.cars)
                && !(memory_sue.perfumes.is_some() && memory_sue.perfumes != actual_sue.perfumes)
        })
        .collect();

    assert!(filtered.len() == 1);
    filtered[0].number.unwrap().to_string()
}

struct Sue {
    number: Option<i32>,
    children: Option<i32>,
    cats: Option<i32>,
    samoyeds: Option<i32>,
    pomeranians: Option<i32>,
    akitas: Option<i32>,
    vizslas: Option<i32>,
    goldfish: Option<i32>,
    trees: Option<i32>,
    cars: Option<i32>,
    perfumes: Option<i32>,
}

impl Sue {
    fn parse_memories(input_memories: &str) -> Vec<Self> {
        let input_memories = input_memories.replace(",", "").replace(":", "");
        input_memories
            .lines()
            .map(|l| {
                //Sue 1: cars: 9, akitas: 3, goldfish: 0
                let parts: Vec<&str> = l.split_whitespace().collect();
                let number = Some(parts[1].parse::<i32>().unwrap());
                let mut children = None;
                let mut cats = None;
                let mut samoyeds = None;
                let mut pomeranians = None;
                let mut akitas = None;
                let mut vizslas = None;
                let mut goldfish = None;
                let mut trees = None;
                let mut cars = None;
                let mut perfumes = None;
                parts
                    .iter()
                    .skip(2)
                    .collect::<Vec<&&str>>()
                    .chunks(2)
                    .for_each(|a| {
                        let val = a[1].parse::<i32>().unwrap();
                        match *a[0] {
                            "children" => children = Some(val),
                            "cats" => cats = Some(val),
                            "samoyeds" => samoyeds = Some(val),
                            "pomeranians" => pomeranians = Some(val),
                            "akitas" => akitas = Some(val),
                            "vizslas" => vizslas = Some(val),
                            "goldfish" => goldfish = Some(val),
                            "trees" => trees = Some(val),
                            "cars" => cars = Some(val),
                            "perfumes" => perfumes = Some(val),
                            e => panic!("unexpected input {}", e),
                        };
                    });
                Self {
                    number,
                    children,
                    cats,
                    samoyeds,
                    pomeranians,
                    akitas,
                    vizslas,
                    goldfish,
                    trees,
                    cars,
                    perfumes,
                }
            })
            .collect()
    }

    fn parse_actual_sue(input_actual_sue: &str) -> Self {
        let mut children = 0;
        let mut cats = 0;
        let mut samoyeds = 0;
        let mut pomeranians = 0;
        let mut akitas = 0;
        let mut vizslas = 0;
        let mut goldfish = 0;
        let mut trees = 0;
        let mut cars = 0;
        let mut perfumes = 0;

        input_actual_sue.lines().for_each(|l| {
            let parts: Vec<&str> = l.split(": ").collect();
            let nb = parts[1].parse::<i32>().unwrap();
            match parts[0] {
                "children" => children = nb,
                "cats" => cats = nb,
                "samoyeds" => samoyeds = nb,
                "pomeranians" => pomeranians = nb,
                "akitas" => akitas = nb,
                "vizslas" => vizslas = nb,
                "goldfish" => goldfish = nb,
                "trees" => trees = nb,
                "cars" => cars = nb,
                "perfumes" => perfumes = nb,
                _ => panic!("unexpected input"),
            };
        });
        Self {
            number: None,
            children: Some(children),
            cats: Some(cats),
            samoyeds: Some(samoyeds),
            pomeranians: Some(pomeranians),
            akitas: Some(akitas),
            vizslas: Some(vizslas),
            goldfish: Some(goldfish),
            trees: Some(trees),
            cars: Some(cars),
            perfumes: Some(perfumes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();
        assert_eq!("103", output);
    }
}
