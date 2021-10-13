use crate::reserve_controller::model::reserve::Reserve;
use crate::reserve_controller::controller::Controller;

pub struct Package {
    origin: String,
    destination: String,
    airline: String,
    hotel: String
}

impl Package {
    pub fn new(origin: String, destination: String, airline: String, hotel: String) -> Package {
        Package{ origin, destination, airline, hotel }
    }
}

impl Reserve for Package {
    fn process(&self, controller: impl Controller + 'static) {
        controller.reserve_package(self.origin.clone(), self.destination.clone(), self.airline.clone(), self.hotel.clone());
    }

}
