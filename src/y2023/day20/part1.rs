use std::collections::HashMap;

fn part_1(input: &str) -> String {
    let modules = parse_input(input);
    dbg!(&modules);
    "".to_string()
}

fn parse_input(input: &str) -> HashMap<String, Module> {
    input
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
                    lasts: destinations
                        .clone()
                        .into_iter()
                        .map(|d| (d, PulseType::Low))
                        .collect(),
                }),
                b'%' => ModuleType::FlipFLop(false),
                _ => ModuleType::Broadcaster,
            };

            (
                name,
                Module {
                    module_type,
                    destinations,
                },
            )
        })
        .collect()
}

#[derive(Debug)]
enum ModuleType {
    FlipFLop(bool),
    Conjunction(LastsPulses),
    Broadcaster,
}

#[derive(Debug)]
struct LastsPulses {
    lasts: HashMap<String, PulseType>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum PulseType {
    Low,
    High,
}

#[derive(Debug)]
struct Module {
    module_type: ModuleType,
    destinations: Vec<String>,
}

impl Module {
    fn handle_pulse(mut self, pulse_type: PulseType, from: String) {
        match (self.module_type, pulse_type) {
            (ModuleType::FlipFLop(_), PulseType::High) => None,
            (ModuleType::FlipFLop(state), PulseType::Low) => {
                self.module_type = ModuleType::FlipFLop(!state);
                if !state {
                    Some(PulseType::High)
                } else {
                    Some(PulseType::Low)
                }
            }
            (ModuleType::Conjunction(mut lasts_pulses), _) => {
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
            (ModuleType::Broadcaster, _) => Some(pulse_type),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("", output);
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
