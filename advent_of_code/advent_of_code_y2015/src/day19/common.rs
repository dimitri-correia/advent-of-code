pub fn parse_input(input: &str) -> (&str, Vec<(&str, &str)>) {
    let mut lines = input.lines();
    let mut replacements = Vec::new();
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(" => ");
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        replacements.push((from, to));
    }
    let start_molecule = lines.next().unwrap();
    (start_molecule, replacements)
}
