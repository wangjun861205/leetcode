use std::collections::HashMap;
struct UndergroundSystem {
    starts: HashMap<i32, (String, i32)>,
    durations: HashMap<(String, String), (i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        UndergroundSystem {
            starts: HashMap::new(),
            durations: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.starts.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let s = self.starts.remove(&id).unwrap();
        let d = self.durations.entry((s.0, station_name)).or_insert((0, 0));
        d.0 += t - s.1;
        d.1 += 1;
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let record = self.durations.get(&(start_station, end_station)).unwrap();
        record.0 as f64 / record.1 as f64
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */
fn main() {
    println!("Hello, world!");
}
