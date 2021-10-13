use crate::reserve_controller::model::reserve::Reserve;
use crate::reserve_controller::controller::Controller;

pub struct Flight {
    origin: String,
    destination: String,
    airline: String
}

impl Flight {
    pub fn new(origin: String, destination: String, airline: String) -> Flight{
        Flight{ origin, destination, airline }
    }
}

impl Reserve for Flight {
    fn process(&self, controller: impl Controller + 'static) {
        controller.reserve_flight(self.origin.clone(), self.destination.clone(), self.airline.clone());
    }

}
