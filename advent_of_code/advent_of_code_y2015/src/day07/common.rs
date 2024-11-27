use std::collections::HashMap;

pub fn parse_input(input: &str) -> HashMap<&str, &str> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let expr = parts.next().unwrap();
            let wire = parts.next().unwrap();
            (wire, expr)
        })
        .collect()
}

pub fn follow_wire(
    wire: &str,
    wires: &HashMap<&str, &str>,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    if let Some(&cached_value) = cache.get(wire) {
        return cached_value;
    }
    let expr = wires.get(wire).unwrap();
    let value = if let Ok(num) = expr.parse::<u16>() {
        num
    } else {
        let parts: Vec<&str> = expr.split_whitespace().collect();
        match parts.as_slice() {
            [x] => resolve(x, wires, cache),
            ["NOT", x] => !resolve(x, wires, cache),
            [x, "AND", y] => resolve(x, wires, cache) & resolve(y, wires, cache),
            [x, "OR", y] => resolve(x, wires, cache) | resolve(y, wires, cache),
            [x, "LSHIFT", y] => resolve(x, wires, cache) << resolve(y, wires, cache),
            [x, "RSHIFT", y] => resolve(x, wires, cache) >> resolve(y, wires, cache),
            _ => panic!("Unknown expression format: {}", expr),
        }
    };

    cache.insert(wire.to_string(), value);
    value
}

fn resolve<'a>(
    part: &str,
    wires: &HashMap<&'a str, &'a str>,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    part.parse::<u16>()
        .unwrap_or_else(|_| follow_wire(part, wires, cache))
}
