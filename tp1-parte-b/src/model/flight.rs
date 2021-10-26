use crate::model::route::Route;
/// It represents a Flight.
pub struct Flight {
    route: Route,
    airline: String,
}

impl Flight {
    pub fn new(route: Route, airline: String) -> Flight {
        Flight {
            route,
            airline,
        }
    }
    pub fn get_route(&self) -> Route {
        self.route.clone()
    }
    pub fn get_airline(&self) -> String {
        self.airline.clone()
    }
}
