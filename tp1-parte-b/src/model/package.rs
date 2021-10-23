use crate::model::route::Route;
pub struct Package {
    route: Route,
    airline: String,
    hotel: String,
}

impl Package {
    pub fn new(route: Route, airline: String, hotel: String) -> Package {
        Package {
            route,
            airline,
            hotel
        }
    }
    pub fn get_route(&self) -> Route {
        self.route.clone()
    }
    pub fn get_airline(&self) -> String {
        self.airline.clone()
    }
    pub fn get_hotel(&self) -> String {
        self.hotel.clone()
    }
}