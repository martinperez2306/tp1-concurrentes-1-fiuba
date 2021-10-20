pub struct Package {
    origin: String,
    destination: String,
    airline: String,
    hotel: String,
}

impl Package {
    pub fn new(origin: String, destination: String, airline: String, hotel: String) -> Package {
        Package {
            origin,
            destination,
            airline,
            hotel,
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
    pub fn get_hotel(&self) -> String {
        self.hotel.clone()
    }
}
