/// It represents a Route between two locations.
pub struct Route {
    origin: String,
    destination: String,
}

impl Route {
    pub fn new(origin: String, destination: String) -> Route {
        Route {
            origin,
            destination,
        }
    }
    pub fn get_id(&self) -> String {
        self.origin.clone() + "_" + &self.destination.clone()
    }
}
