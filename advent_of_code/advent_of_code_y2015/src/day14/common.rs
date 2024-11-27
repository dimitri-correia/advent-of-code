pub fn parse_input(input: &str) -> Vec<Reindeer> {
    input
        .lines()
        .map(|line| {
            // ex : Rudolph can fly 22 km/s for 8 seconds, but then must rest for 165 seconds.
            let parts: Vec<&str> = line.split_whitespace().collect();
            assert!(parts[1] == "can" && parts[2] == "fly");
            let speed_in_km_s = parts[3].parse::<u16>().unwrap();
            assert!(parts[4] == "km/s");
            let fly_time_in_s = parts[6].parse::<u16>().unwrap();
            assert!(parts[7] == "seconds,");
            let rest_time_in_s = parts[13].parse::<u16>().unwrap();
            assert!(parts[14] == "seconds.");
            Reindeer {
                speed_in_km_s,
                fly_time_in_s,
                rest_time_in_s,
            }
        })
        .collect()
}

pub struct Reindeer {
    speed_in_km_s: u16,
    fly_time_in_s: u16,
    rest_time_in_s: u16,
}

impl Reindeer {
    pub fn distance_after(&self, time_in_s: u16) -> u16 {
        let cycle_time_in_s = self.fly_time_in_s + self.rest_time_in_s;
        let nb_cycles = time_in_s / cycle_time_in_s;
        let remaining_time_in_s = time_in_s % cycle_time_in_s;
        let fly_time_not_complete_cycle_in_s =
            std::cmp::min(remaining_time_in_s, self.fly_time_in_s);
        let distance_in_km = (nb_cycles * self.speed_in_km_s * self.fly_time_in_s)
            + (fly_time_not_complete_cycle_in_s * self.speed_in_km_s);
        distance_in_km
    }
}
