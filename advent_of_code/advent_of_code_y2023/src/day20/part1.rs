use std::collections::HashMap;

pub fn part_1_example_1() -> String {
    let input = include_str!("input1_ex1.txt");
    part_1(input)
}

pub fn part_1_example_2() -> String {
    let input = include_str!("input1_ex2.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let mut modules = parse_input(input);

    let mut next_step: Vec<(String, PulseType, String)> = vec![];

    let mut count_low = 0;
    let mut count_high = 0;

    for _a in 0..1000 {
        handle_one_button_press(
            &mut modules,
            &mut next_step,
            &mut count_low,
            &mut count_high,
        );
    }

    (count_low * count_high).to_string()
}

fn handle_one_button_press(
    modules: &mut HashMap<String, Module>,
    next_step: &mut Vec<(String, PulseType, String)>,
    count_low: &mut i32,
    count_high: &mut i32,
) {
    *count_low += 1; //button -low-> broadcaster

    generate_next_steps(
        "roadcaster",
        PulseType::Low,
        modules,
        next_step,
        count_low,
        count_high,
    );

    while !next_step.is_empty() {
        let (module_name, pulse_type, from) = next_step.remove(0);

        if let Some(module) = modules.get_mut(&module_name) {
            if let Some(new_pulse) = module.handle_pulse(pulse_type, from.clone()) {
                generate_next_steps(
                    &*module_name,
                    new_pulse,
                    modules,
                    next_step,
                    count_low,
                    count_high,
                );
            }
        }
    }
}

fn generate_next_steps(
    name: &str,
    pulse_type: PulseType,
    modules: &mut HashMap<String, Module>,
    next_step: &mut Vec<(String, PulseType, String)>,
    count_low: &mut i32,
    count_high: &mut i32,
) {
    modules
        .get(name)
        .unwrap()
        .destinations
        .iter()
        .for_each(|dest| {
            match pulse_type {
                PulseType::Low => *count_low += 1,
                PulseType::High => *count_high += 1,
            };
            next_step.push((dest.clone(), pulse_type, name.to_string()));
        });
}

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFLop(bool),
    Conjunction(LastsPulses),
    Broadcaster,
}

#[derive(Debug, Clone)]
struct LastsPulses {
    lasts: HashMap<String, PulseType>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum PulseType {
    Low,
    High,
}

#[derive(Debug, Clone)]
struct Module {
    module_type: ModuleType,
    destinations: Vec<String>,
}

impl Module {
    fn handle_pulse(&mut self, pulse_type: PulseType, from: String) -> Option<PulseType> {
        match &mut self.module_type {
            ModuleType::FlipFLop(_) if pulse_type == PulseType::High => None,
            ModuleType::FlipFLop(state) => {
                // pulse_type == PulseType::Low
                *state = !*state;
                // If it was off, it turns on and sends a high pulse.
                // If it was on, it turns off and sends a low pulse.
                if *state {
                    Some(PulseType::High)
                } else {
                    Some(PulseType::Low)
                }
            }
            ModuleType::Conjunction(lasts_pulses) => {
                *lasts_pulses.lasts.get_mut(&from).unwrap() = pulse_type;
                if lasts_pulses
                    .lasts
                    .iter()
                    .all(|(_, p)| p == &PulseType::High)
                {
                    Some(PulseType::Low)
                } else {
                    Some(PulseType::High)
                }
            }
            ModuleType::Broadcaster => Some(pulse_type),
        }
    }
}

fn parse_input(input: &str) -> HashMap<String, Module> {
    let mut tmp: HashMap<String, Vec<String>> = HashMap::new();

    let parsed: HashMap<String, Module> = input
        .lines()
        .map(|l| {
            let mut parts = l.trim().split("->");
            let source_dest = parts.next().unwrap().trim();
            let name = (&source_dest[1..]).to_string();
            let destinations: Vec<String> = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            let module_type = match source_dest.chars().next().unwrap() {
                '&' => ModuleType::Conjunction(LastsPulses {
                    lasts: HashMap::new(),
                }),
                '%' => ModuleType::FlipFLop(false),
                _ => ModuleType::Broadcaster,
            };

            destinations.iter().for_each(|d| {
                tmp.entry(d.clone())
                    .or_insert_with(Vec::new)
                    .push(name.clone());
            });

            (
                name,
                Module {
                    module_type,
                    destinations,
                },
            )
        })
        .collect();

    parsed
        .into_iter()
        .map(|(name, mut module)| {
            let new_module_type = match &mut module.module_type {
                ModuleType::Conjunction(_) => {
                    let lasts = tmp
                        .get(&name)
                        .unwrap()
                        .iter()
                        .map(|n| (n.clone(), PulseType::Low))
                        .collect();
                    ModuleType::Conjunction(LastsPulses { lasts })
                }
                _ => module.module_type.clone(),
            };
            module.module_type = new_module_type;
            (name, module)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("739960225", output);
    }

    #[test]
    fn example_test1() {
        let r = part_1_example_1();
        assert_eq!("32000000", r);
    }

    #[test]
    fn example_test2() {
        let r = part_1_example_2();
        assert_eq!("11687500", r);
    }
}
