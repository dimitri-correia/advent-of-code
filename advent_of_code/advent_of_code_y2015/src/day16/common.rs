pub struct Sue {
    pub number: Option<i32>,
    pub children: Option<i32>,
    pub cats: Option<i32>,
    pub samoyeds: Option<i32>,
    pub pomeranians: Option<i32>,
    pub akitas: Option<i32>,
    pub vizslas: Option<i32>,
    pub goldfish: Option<i32>,
    pub trees: Option<i32>,
    pub cars: Option<i32>,
    pub perfumes: Option<i32>,
}

impl Sue {
    pub fn parse_memories(input_memories: &str) -> Vec<Self> {
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

    pub fn parse_actual_sue(input_actual_sue: &str) -> Self {
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
