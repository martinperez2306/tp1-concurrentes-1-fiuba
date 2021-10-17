use std::collections::HashMap;
use crate::reserve_controller::model::route::Route;

pub struct Stats {
    routes: HashMap<String, u32>
}

impl Stats {
    pub fn new() -> Stats {
        let routes = HashMap::new();
        Stats{ routes }
    }
    pub fn increment_route_counter(&mut self, route: Route) {
        let route_id = route.get_id();
        let count = self.routes.entry(route_id).or_insert(0);
        *count += 1;
    }
    pub fn get_routes(&self) -> HashMap<String, u32> { self.routes.clone() }
}