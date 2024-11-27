use std::collections::HashMap;

pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    parse_input(input)
        .iter()
        .fold(HashMap::new(), |mut boxes, step| {
            let lens_box: &mut Vec<Lens> = boxes.entry(compute_box_label(step)).or_default();

            if step.label.contains('-') {
                lens_box.retain(|b| !step.label.contains(&b.label));
                return boxes;
            }

            if let Some(existing_lens) = lens_box.iter_mut().find(|l| l.label == step.label) {
                existing_lens.focal_length = step.focal_length;
            } else {
                lens_box.push(step.clone());
            }

            boxes
        })
        .iter()
        .map(|(label, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(pos, lens)| (label + 1) * (pos + 1) * lens.focal_length)
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn compute_box_label(step: &Lens) -> usize {
    step.label
        .chars()
        .filter(|c| c.is_alphabetic())
        .fold(0, |acc, next_char| (acc + (next_char as usize)) * 17 % 256)
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

fn parse_input(input: &str) -> Vec<Lens> {
    input
        .split(',')
        .map(|part| {
            let mut sub_parts = part.split('=');
            Lens {
                label: sub_parts.next().unwrap().to_string(),
                focal_length: sub_parts.next().unwrap_or("0").parse().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("286097", output);
    }

    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("145", r);
    }
}
