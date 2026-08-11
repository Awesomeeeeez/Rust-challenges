// https://leetcode.com/problems/design-underground-system/description/?envType=problem-list-v2&envId=design

use std::{collections::HashMap, println};

struct UndergroundSystem {
    process: HashMap<i32, (String, i32)>,
    total: HashMap<String, Vec<(String, i32)>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {

    fn new() -> Self {
        Self {
            process: HashMap::new(),
            total: HashMap::new(),
        }
    }
    
    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.process.insert(id, (station_name, t));
    }
    
    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        self.total
            .entry(self.process.get(&id).unwrap().0.clone())
            .or_insert(Vec::new())
            .push((station_name, t - self.process.get(&id).unwrap().1));
        self.process.remove(&id);
    }
    
    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let mut time_sum = 0;
        let mut count = 0;
        for i in self.total.get(&start_station.clone()).unwrap() {
            if *i.0 == end_station {
                count += 1;
                time_sum += i.1;
            }
        }
        time_sum as f64 / count as f64
    }
}

fn main() {
    let mut us = UndergroundSystem::new();
us.check_in(1, "Loop".to_string(), 0);
us.check_out(1, "Loop".to_string(), 5); // заехал и вышел на той же станции
println!("{}", us.get_average_time("Loop".to_string(), "Loop".to_string())); // 5.0
}