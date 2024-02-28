use std::collections::HashMap;

fn part_1(input: &str) -> String {
    let mut modules = parse_input(input);

    let mut next_step: Vec<(String, PulseType, String)> = vec![];

    let mut count_low = 0;
    let mut count_high = 0;

    for _a in 0..1000 {
        count_low += 1; //button -low-> broadcaster
        modules
            .get("roadcaster")
            .unwrap()
            .destinations
            .iter()
            .for_each(|dest| {
                match PulseType::Low {
                    PulseType::Low => count_low += 1,
                    PulseType::High => count_high += 1,
                };
                next_step.push((dest.clone(), PulseType::Low, "roadcaster".to_string()));
            });

        while !next_step.is_empty() {
            let (module_name, pulse_type, from) = next_step.remove(0);

            if !modules.contains_key(&module_name) {
                continue;
            }

            let new_pulse = modules
                .get_mut(&module_name)
                .unwrap()
                .handle_pulse(pulse_type, from.clone());

            if new_pulse.is_none() {
                continue;
            }
            let new_pulse = new_pulse.unwrap();

            modules
                .get(&module_name)
                .unwrap()
                .destinations
                .iter()
                .for_each(|dest| {
                    match new_pulse {
                        PulseType::Low => count_low += 1,
                        PulseType::High => count_high += 1,
                    };
                    next_step.push((dest.clone(), new_pulse, module_name.clone()));
                })
        }
    }

    (count_low * count_high).to_string()
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
            let module_type = match source_dest.as_bytes()[0] {
                b'&' => ModuleType::Conjunction(LastsPulses {
                    lasts: HashMap::new(),
                }),
                b'%' => ModuleType::FlipFLop(false),
                _ => ModuleType::Broadcaster,
            };

            destinations.clone().into_iter().for_each(|d| {
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
        .map(|(name, mut module)| match &mut module.module_type {
            ModuleType::Conjunction(_) => (
                name.clone(),
                Module {
                    module_type: ModuleType::Conjunction(LastsPulses {
                        lasts: tmp
                            .get(&(name.clone()))
                            .unwrap()
                            .iter()
                            .flat_map(|n| vec![(n.clone(), PulseType::Low)])
                            .collect(),
                    }),
                    destinations: module.destinations,
                },
            ),
            _ => (name, module),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("739960225", output);
    }

    #[test]
    fn example_test1() {
        let input = include_str!("input1_ex1.txt");
        let r = part_1(input);
        assert_eq!("32000000", r);
    }

    #[test]
    fn example_test2() {
        let input = include_str!("input1_ex2.txt");
        let r = part_1(input);
        assert_eq!("11687500", r);
    }
}
