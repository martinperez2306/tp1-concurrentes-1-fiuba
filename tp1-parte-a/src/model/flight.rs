/// It represents a Flight reserve and it has the needed information for it
pub struct Flight {
    origin: String,
    destination: String,
    airline: String,
}

impl Flight {
    pub fn new(origin: String, destination: String, airline: String) -> Flight {
        Flight {
            origin,
            destination,
            airline,
        }
    }
    pub fn get_origin(&self) -> String {
        self.origin.clone()
    }
    pub fn get_destination(&self) -> String {
        self.destination.clone()
    }
    pub fn get_airline(&self) -> String {
        self.airline.clone()
    }
}
