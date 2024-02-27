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
            let module_type = match source_dest.as_bytes()[0] {
                b'&' => ModuleType::Conjunction,
                b'%' => ModuleType::FlipFLop,
                _ => ModuleType::Broadcaster,
            };
            let name = (&source_dest[1..]).to_string();
            let destinations: Vec<String> = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

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
    FlipFLop,
    Conjunction,
    Broadcaster,
}

#[derive(Debug)]
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
    fn handle_pulse(self: Self, pulse_type: PulseType) {
        match (self.module_type, pulse_type) {
            (ModuleType::FlipFLop, PulseType::High) => {}
            (ModuleType::FlipFLop, PulseType::Low) => {}
            (ModuleType::Conjunction, PulseType::High) => {}
            (ModuleType::Conjunction, PulseType::Low) => {}
            (ModuleType::Broadcaster, PulseType::High) => {}
            (ModuleType::Broadcaster, PulseType::Low) => {}
        }
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
