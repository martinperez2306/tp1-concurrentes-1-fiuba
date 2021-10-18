use std::collections::HashMap;
use crate::reserve_controller::model::route::Route;

pub struct Stats {
    routes: HashMap<String, u32>,
    reserve_processing_times: Vec<u64>
}

impl Stats {
    pub fn new() -> Stats {
        let routes = HashMap::new();
        let reserve_processing_times = vec![];
        Stats{ routes, reserve_processing_times }
    }
    pub fn increment_route_counter(&mut self, route: Route) {
        let route_id = route.get_id();
        let count = self.routes.entry(route_id).or_insert(0);
        *count += 1;
    }
    pub fn get_routes(&self) -> HashMap<String, u32> { self.routes.clone() }
    fn get_reserve_processing_time(&self) -> Vec<u64> { self.reserve_processing_times.clone() }
    pub fn add_reserve_processing_time(&mut self, time: u64){
        self.reserve_processing_times.push(time);
    }
    pub fn get_avg_reserve_processing_time(&self) -> u64 {
        let mut avg: u64 = 0;
        let mut count: u64 = 0;
        for time in self.get_reserve_processing_time() {
            count += 1;
            avg += time;
        }
        avg = avg / count;
        return avg;
    }
}