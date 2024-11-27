pub fn parse_input(input: &str) -> Vec<Ingredient> {
    let input = input.replace(",", "");
    input
        .trim()
        .lines()
        .map(|l| {
            // ex Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
            let parts: Vec<&str> = l.split_whitespace().collect();
            assert!(parts[1] == "capacity");
            let capacity = parts[2].parse::<i32>().unwrap();
            assert!(parts[3] == "durability");
            let durability = parts[4].parse::<i32>().unwrap();
            assert!(parts[5] == "flavor");
            let flavor = parts[6].parse::<i32>().unwrap();
            assert!(parts[7] == "texture");
            let texture = parts[8].parse::<i32>().unwrap();
            assert!(parts[9] == "calories");
            let calories = parts[10].parse::<i32>().unwrap();
            Ingredient {
                capacity,
                durability,
                flavor,
                texture,
                calories,
            }
        })
        .collect()
}

#[derive(Clone)]
pub struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

#[derive(Clone)]
pub struct Receipe {
    pub ingredients: Vec<(Ingredient, i32)>,
}

impl Receipe {
    pub fn compute_total_score(&self, part: Part) -> i32 {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;

        assert!(self.ingredients.iter().map(|(_, q)| q).sum::<i32>() == 100);

        for (ingredient, quantity) in &self.ingredients {
            capacity += ingredient.capacity * (*quantity);
            durability += ingredient.durability * (*quantity);
            flavor += ingredient.flavor * (*quantity);
            texture += ingredient.texture * (*quantity);
        }

        if part.eq(&Part::Part2) {
            let calories = self
                .ingredients
                .iter()
                .map(|(i, q)| i.calories * q)
                .sum::<i32>();
            if calories != 500 {
                return 0;
            }
        }

        if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
            return 0;
        }
        capacity * durability * flavor * texture
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Part {
    Part1,
    Part2,
}
