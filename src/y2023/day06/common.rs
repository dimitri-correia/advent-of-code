pub fn calculate_local_res(total_time: usize, distance_to_beat: usize) -> i32 {
    (0..=total_time)
        .filter(|&charging_time| {
            let speed = charging_time;
            let time_of_travel = total_time - charging_time;
            let dist = speed * time_of_travel;
            dist > distance_to_beat
        })
        .count() as i32
}
