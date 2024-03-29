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
    pub fn get_origin(&self) -> String {
        self.origin.clone()
    }
    pub fn get_destination(&self) -> String {
        self.destination.clone()
    }
    pub fn get_id(&self) -> String {
        self.origin.clone() + "_" + &self.destination.clone()
    }
    pub fn clone(&self) -> Route {
        Route::new(self.origin.clone(), self.destination.clone())
    }
}
