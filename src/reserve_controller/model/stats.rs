use std::collections::HashMap;

pub struct Stats {
    routes: HashMap<String, u32>
}

impl Stats {
    pub fn new() -> Stats {
        let routes = HashMap::new();
        Stats{ routes }
    }
    pub fn increment_route_counter(mut self, origin: String, destination: String) {
        let route = origin.clone() + "_" + &destination;
        let count = self.routes.entry(route).or_insert(0);
        *count += 1;
    }
}