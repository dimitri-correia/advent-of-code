use std::collections::HashMap;

fn part_2(input: &str) -> String {
    let mut boxes: HashMap<usize, Vec<Lens>> = HashMap::new();

    let steps = parse_input(input);

    for step in &steps {
        let lens_box = boxes.entry(compute_box_label(step)).or_default();

        if step.label.contains('-') {
            lens_box.retain(|b| !step.label.contains(&b.label));
            continue;
        }

        if let Some(existing_lens) = lens_box.iter_mut().find(|l| l.label == step.label) {
            existing_lens.focal_length = step.focal_length;
        } else {
            lens_box.push(step.clone());
        }
    }

    let mut r = 0;
    for (label, lenses) in boxes {
        let label = label + 1;
        for (pos, lens) in lenses.iter().enumerate() {
            r += label * (pos + 1) * lens.focal_length;
        }
    }

    r.to_string()
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
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("110407", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_2(input);
        assert_eq!("145", r);
    }
}
